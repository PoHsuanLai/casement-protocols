//! Output dispatch: the manager, heads, modes and configurations (`zwlr-output-management-v1`).

use wayland_client::protocol::wl_output;
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle, event_created_child};
use wayland_protocols_wlr::output_management::v1::client::{
    zwlr_output_configuration_head_v1::{self, ZwlrOutputConfigurationHeadV1},
    zwlr_output_configuration_v1::{self, ZwlrOutputConfigurationV1},
    zwlr_output_head_v1::{self, ZwlrOutputHeadV1},
    zwlr_output_manager_v1::{self, ZwlrOutputManagerV1},
    zwlr_output_mode_v1::{self, ZwlrOutputModeV1},
};

use super::error::WireError;
use super::wire::{Wire, number_of, raw};
use crate::command::CommandKind;
use crate::error::CommandError;
use crate::event::DesktopEvent;
use crate::geometry::{Point, Size};
use crate::keys::{ConfigId, HeadKey, ModeKey};
use crate::names::OutputName;
use crate::output::{
    AdaptiveSync, ConfigError, ConfigOutcome, ConfigPurpose, Enabled, HeadChange,
    HeadChangeRequest, HeadSettings, ModeChange, ModeChoice, OutputConfig, OutputEvent, Preferred,
    RefreshMilliHz, ScaleMilli, Transform, complete,
};
use crate::protocol::Protocol;

fn output_event(wire: &mut Wire, event: OutputEvent) {
    wire.emit(DesktopEvent::Output(event));
}

fn head_change(wire: &mut Wire, key: HeadKey, change: HeadChange) {
    output_event(wire, OutputEvent::Head { key, change });
}

fn mode_change(wire: &mut Wire, mode: ModeKey, change: ModeChange) {
    output_event(wire, OutputEvent::Mode { mode, change });
}

/// Millimetres or pixels off the wire; a negative value reads as 0.
fn extent(value: i32) -> u32 {
    u32::try_from(value).unwrap_or(0)
}

/// The manager's event.
pub(super) fn on_manager(wire: &mut Wire, event: zwlr_output_manager_v1::Event) {
    use zwlr_output_manager_v1::Event;
    match event {
        Event::Head { head } => {
            let key = HeadKey::new(number_of(&head));
            wire.handles.heads.insert(key, head);
            output_event(wire, OutputEvent::HeadOpened(key));
        }
        Event::Done { serial } => output_event(wire, OutputEvent::Done { serial }),
        Event::Finished => output_event(wire, OutputEvent::Finished),
        _ => {}
    }
}

/// A head's event.
pub(super) fn on_head(wire: &mut Wire, key: HeadKey, event: zwlr_output_head_v1::Event) {
    use zwlr_output_head_v1::Event;
    match event {
        Event::Name { name } => head_change(wire, key, HeadChange::Name(OutputName::new(name))),
        Event::Description { description } => {
            head_change(wire, key, HeadChange::Description(description));
        }
        Event::PhysicalSize { width, height } => {
            head_change(
                wire,
                key,
                HeadChange::PhysicalSize(Size::new(extent(width), extent(height))),
            );
        }
        Event::Mode { mode } => {
            let mode_key = ModeKey::new(number_of(&mode));
            wire.handles.modes.insert(mode_key, mode);
            output_event(
                wire,
                OutputEvent::ModeOpened {
                    head: key,
                    mode: mode_key,
                },
            );
        }
        Event::Enabled { enabled } => {
            head_change(wire, key, HeadChange::Enabled(Enabled::from_wire(enabled)));
        }
        Event::CurrentMode { mode } => {
            head_change(
                wire,
                key,
                HeadChange::CurrentMode(ModeKey::new(number_of(&mode))),
            );
        }
        Event::Position { x, y } => head_change(wire, key, HeadChange::Position(Point::new(x, y))),
        Event::Transform { transform } => {
            if let Some(transform) = Transform::from_code(raw(transform)) {
                head_change(wire, key, HeadChange::Transform(transform));
            }
        }
        Event::Scale { scale } => {
            head_change(wire, key, HeadChange::Scale(ScaleMilli::from_f64(scale)));
        }
        Event::Make { make } => head_change(wire, key, HeadChange::Make(make)),
        Event::Model { model } => head_change(wire, key, HeadChange::Model(model)),
        Event::SerialNumber { serial_number } => {
            head_change(wire, key, HeadChange::SerialNumber(serial_number));
        }
        Event::AdaptiveSync { state } => {
            head_change(
                wire,
                key,
                HeadChange::AdaptiveSync(AdaptiveSync::from_code(raw(state))),
            );
        }
        Event::Finished => {
            output_event(wire, OutputEvent::HeadFinished(key));
            if let Some(head) = wire.handles.heads.remove(&key)
                && head.version() >= zwlr_output_head_v1::REQ_RELEASE_SINCE
            {
                head.release();
            }
        }
        _ => {}
    }
}

/// A mode's event.
pub(super) fn on_mode(wire: &mut Wire, key: ModeKey, event: zwlr_output_mode_v1::Event) {
    use zwlr_output_mode_v1::Event;
    match event {
        Event::Size { width, height } => {
            mode_change(
                wire,
                key,
                ModeChange::Size(Size::new(extent(width), extent(height))),
            );
        }
        Event::Refresh { refresh } => {
            mode_change(
                wire,
                key,
                ModeChange::Refresh(RefreshMilliHz::new(extent(refresh))),
            );
        }
        Event::Preferred => mode_change(wire, key, ModeChange::Preferred(Preferred::Yes)),
        Event::Finished => {
            mode_change(wire, key, ModeChange::Finished);
            if let Some(mode) = wire.handles.modes.remove(&key)
                && mode.version() >= zwlr_output_mode_v1::REQ_RELEASE_SINCE
            {
                mode.release();
            }
        }
        _ => {}
    }
}

/// A configuration's answer.
pub(super) fn on_configuration(
    wire: &mut Wire,
    id: ConfigId,
    purpose: ConfigPurpose,
    configuration: &ZwlrOutputConfigurationV1,
    event: zwlr_output_configuration_v1::Event,
) {
    use zwlr_output_configuration_v1::Event;
    let outcome = match event {
        Event::Succeeded => ConfigOutcome::Succeeded,
        Event::Failed => ConfigOutcome::Failed,
        Event::Cancelled => ConfigOutcome::Cancelled,
        _ => return,
    };
    output_event(
        wire,
        OutputEvent::ConfigResolved {
            id,
            purpose,
            outcome,
        },
    );
    configuration.destroy();
}

/// Builds and sends a `create_configuration` for `config` quoting the state's serial, mentioning
/// every enabled head the config leaves out, and applies or tests it.
pub(super) fn submit(
    wire: &mut Wire,
    config: &OutputConfig,
    purpose: ConfigPurpose,
) -> Result<ConfigId, WireError> {
    config
        .validate(&wire.state.outputs)
        .map_err(CommandError::Config)?;
    let serial = wire
        .state
        .outputs
        .serial()
        .ok_or(CommandError::Config(ConfigError::NotSettled))?;
    let command = match purpose {
        ConfigPurpose::Apply => CommandKind::ApplyOutputs,
        ConfigPurpose::Test => CommandKind::TestOutputs,
    };
    let manager = wire
        .handles
        .output_manager
        .clone()
        .ok_or(CommandError::Unsupported {
            command,
            needs: Protocol::WlrOutputManagement,
        })?;
    let parts = complete(config, &wire.state.outputs);
    let id = ConfigId::new(wire.mint());
    let configuration = manager.create_configuration(serial, &wire.qh, (id, purpose));
    for part in &parts {
        let Some(head) = wire.handles.heads.get(&part.head) else {
            continue;
        };
        match &part.request {
            HeadChangeRequest::Disable => configuration.disable_head(head),
            HeadChangeRequest::Enable(settings) => {
                let target = configuration.enable_head(head, &wire.qh, ());
                set_head(wire, &target, settings);
            }
        }
    }
    match purpose {
        ConfigPurpose::Apply => configuration.apply(),
        ConfigPurpose::Test => configuration.test(),
    }
    Ok(id)
}

/// Sends the settings of one enabled head.
fn set_head(wire: &Wire, target: &ZwlrOutputConfigurationHeadV1, settings: &HeadSettings) {
    match &settings.mode {
        Some(ModeChoice::Existing(key)) => {
            if let Some(mode) = wire.handles.modes.get(key) {
                target.set_mode(mode);
            }
        }
        Some(ModeChoice::Custom { size, refresh }) => {
            target.set_custom_mode(
                i32::try_from(size.w).unwrap_or(i32::MAX),
                i32::try_from(size.h).unwrap_or(i32::MAX),
                i32::try_from(refresh.get()).unwrap_or(i32::MAX),
            );
        }
        None => {}
    }
    if let Some(position) = settings.position {
        target.set_position(position.x, position.y);
    }
    if let Some(transform) = settings
        .transform
        .and_then(|t| wl_output::Transform::try_from(t.code()).ok())
    {
        target.set_transform(transform);
    }
    if let Some(scale) = settings.scale {
        target.set_scale(scale.to_f64());
    }
    if let Some(sync) = settings.adaptive_sync
        && target.version() >= zwlr_output_configuration_head_v1::REQ_SET_ADAPTIVE_SYNC_SINCE
        && let Ok(state) = zwlr_output_head_v1::AdaptiveSyncState::try_from(sync.code())
    {
        target.set_adaptive_sync(state);
    }
}

impl Dispatch<ZwlrOutputManagerV1, ()> for Wire {
    fn event(
        state: &mut Self,
        _proxy: &ZwlrOutputManagerV1,
        event: zwlr_output_manager_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_manager(state, event);
    }

    event_created_child!(Wire, ZwlrOutputManagerV1, [
        zwlr_output_manager_v1::EVT_HEAD_OPCODE => (ZwlrOutputHeadV1, ()),
    ]);
}

impl Dispatch<ZwlrOutputHeadV1, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &ZwlrOutputHeadV1,
        event: zwlr_output_head_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_head(state, HeadKey::new(number_of(proxy)), event);
    }

    event_created_child!(Wire, ZwlrOutputHeadV1, [
        zwlr_output_head_v1::EVT_MODE_OPCODE => (ZwlrOutputModeV1, ()),
    ]);
}

impl Dispatch<ZwlrOutputModeV1, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &ZwlrOutputModeV1,
        event: zwlr_output_mode_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_mode(state, ModeKey::new(number_of(proxy)), event);
    }
}

impl Dispatch<ZwlrOutputConfigurationV1, (ConfigId, ConfigPurpose)> for Wire {
    fn event(
        state: &mut Self,
        proxy: &ZwlrOutputConfigurationV1,
        event: zwlr_output_configuration_v1::Event,
        data: &(ConfigId, ConfigPurpose),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_configuration(state, data.0, data.1, proxy, event);
    }
}

impl Dispatch<ZwlrOutputConfigurationHeadV1, ()> for Wire {
    fn event(
        _state: &mut Self,
        _proxy: &ZwlrOutputConfigurationHeadV1,
        _event: zwlr_output_configuration_head_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}
