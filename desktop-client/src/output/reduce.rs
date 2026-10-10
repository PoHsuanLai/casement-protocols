//! The output transition.

use super::event::{HeadChange, ModeChange, OutputEvent};
use super::model::{Head, Mode};
use super::set::Outputs;
use super::units::Enabled;

/// `outputs` after `event`. A change to an unknown key is ignored; `Done` records the serial;
/// `ConfigResolved` does not touch the heads (the compositor sends the new head state next).
/// `Finished` drops every head and the serial.
pub(crate) fn reduce_outputs(mut outputs: Outputs, event: OutputEvent) -> Outputs {
    match event {
        OutputEvent::HeadOpened(key) => {
            outputs.heads.entry(key).or_insert_with(|| Head::new(key));
        }
        OutputEvent::Head { key, change } => {
            if let Some(head) = outputs.heads.get_mut(&key) {
                apply_head(head, change);
            }
        }
        OutputEvent::HeadFinished(key) => {
            outputs.heads.remove(&key);
        }
        OutputEvent::ModeOpened { head, mode } => {
            let known = outputs
                .heads
                .values()
                .any(|h| h.modes.iter().any(|m| m.key == mode));
            if let (Some(head), false) = (outputs.heads.get_mut(&head), known) {
                head.modes.push(Mode::new(mode));
            }
        }
        OutputEvent::Mode { mode, change } => apply_mode(&mut outputs, mode, change),
        OutputEvent::Done { serial } => outputs.serial = Some(serial),
        OutputEvent::Finished => {
            outputs.heads.clear();
            outputs.serial = None;
        }
        OutputEvent::ConfigResolved { .. } => {}
    }
    outputs
}

fn apply_head(head: &mut Head, change: HeadChange) {
    match change {
        HeadChange::Name(name) => head.name = name,
        HeadChange::Description(description) => head.description = description,
        HeadChange::PhysicalSize(size) => head.physical_size_mm = Some(size),
        HeadChange::Make(make) => head.make = Some(make),
        HeadChange::Model(model) => head.model = Some(model),
        HeadChange::SerialNumber(serial) => head.serial_number = Some(serial),
        HeadChange::Enabled(enabled) => {
            head.enabled = enabled;
            if enabled == Enabled::Off {
                head.current_mode = None;
            }
        }
        HeadChange::CurrentMode(mode) => head.current_mode = Some(mode),
        HeadChange::Position(position) => head.position = position,
        HeadChange::Transform(transform) => head.transform = transform,
        HeadChange::Scale(scale) => head.scale = scale,
        HeadChange::AdaptiveSync(sync) => head.adaptive_sync = Some(sync),
    }
}

fn apply_mode(outputs: &mut Outputs, key: crate::keys::ModeKey, change: ModeChange) {
    for head in outputs.heads.values_mut() {
        let Some(index) = head.modes.iter().position(|m| m.key == key) else {
            continue;
        };
        match change {
            ModeChange::Size(size) => head.modes[index].size = size,
            ModeChange::Refresh(refresh) => head.modes[index].refresh = refresh,
            ModeChange::Preferred(preferred) => head.modes[index].preferred = preferred,
            ModeChange::Finished => {
                head.modes.remove(index);
                if head.current_mode == Some(key) {
                    head.current_mode = None;
                }
            }
        }
        return;
    }
}
