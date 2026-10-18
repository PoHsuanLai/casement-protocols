//! Quire dispatch: the desktop manager and its apps, windows and Spaces.
//!
//! The manager announces every window as an object of its own. Each is handed to the
//! [`crate::quire::Linker`], which matches it with the ext handle of the same identifier; the
//! window's events reach the state only under its toplevel's key.

use wayland_client::{Connection, Dispatch, QueueHandle, event_created_child};

use casement_protocols::client::quire_desktop_v1::{
    quire_desktop_app_v1::{self, QuireDesktopAppV1},
    quire_desktop_manager_v1::{self, QuireDesktopManagerV1},
    quire_desktop_space_v1::{self, QuireDesktopSpaceV1},
    quire_desktop_window_v1::{self, QuireDesktopWindowV1},
};
use casement_protocols::vocab::{ActorKind, GlowCode, PlaceKind, ProofKind};

use super::wire::{Wire, number_of, vocab};
use crate::event::DesktopEvent;
use crate::geometry::Rect;
use crate::keys::{AppKey, SpaceKey};
use crate::names::{AppId, AppSubject, SpaceId, ToplevelIdentifier};
use crate::quire::{
    AppChange, AppEvent, Lease, MenuSource, QuireEvent, SpaceChange, SpaceEvent, SpacePlace,
    WindowChange, WindowGeometry, WindowObject,
};

fn emit(wire: &mut Wire, event: QuireEvent) {
    wire.emit(DesktopEvent::Quire(event));
}

fn emit_all(wire: &mut Wire, events: Vec<QuireEvent>) {
    for event in events {
        emit(wire, event);
    }
}

/// The manager's events: new objects, the active pair, `finished`.
pub(super) fn on_manager(
    wire: &mut Wire,
    proxy: &QuireDesktopManagerV1,
    event: quire_desktop_manager_v1::Event,
) {
    use quire_desktop_manager_v1::Event;
    match event {
        Event::App { id } => {
            let key = AppKey::new(number_of(&id));
            wire.handles.apps.insert(key, id);
            emit(wire, QuireEvent::App(AppEvent::Opened(key)));
        }
        Event::Window { id } => {
            wire.handles
                .windows
                .insert(WindowObject::new(number_of(&id)), id);
        }
        Event::Space { id } => {
            let key = SpaceKey::new(number_of(&id));
            wire.handles.spaces.insert(key, id);
            emit(wire, QuireEvent::Space(SpaceEvent::Opened(key)));
        }
        Event::Active { app, key, actor } => {
            let app = app.as_ref().map(|a| AppKey::new(number_of(a)));
            let key = key
                .as_ref()
                .and_then(|w| wire.linker.key_of(WindowObject::new(number_of(w))));
            let actor = vocab(actor).unwrap_or(ActorKind::Unknown);
            emit(wire, QuireEvent::Active { app, key, actor });
        }
        Event::Finished => {
            proxy.destroy();
            wire.handles.quire_manager = None;
        }
        _ => {}
    }
}

fn app_change(wire: &mut Wire, proxy: &QuireDesktopAppV1, change: Option<AppChange>) {
    if let Some(change) = change {
        let key = AppKey::new(number_of(proxy));
        emit(wire, QuireEvent::App(AppEvent::Changed { key, change }));
    }
}

/// An app's events.
pub(super) fn on_app(
    wire: &mut Wire,
    proxy: &QuireDesktopAppV1,
    event: quire_desktop_app_v1::Event,
) {
    use quire_desktop_app_v1::Event;
    let key = AppKey::new(number_of(proxy));
    let change = match event {
        Event::Identity {
            key: id,
            proof,
            claimed_id,
        } => Some(AppChange::Identity {
            id: AppId::new(id),
            proof: vocab(proof).unwrap_or(ProofKind::Claimed),
            claimed_id,
        }),
        Event::Subject { subject } => Some(AppChange::Subject(AppSubject::new(subject))),
        Event::Life { state } => vocab(state).map(AppChange::Life),
        Event::Visibility { state } => vocab(state).map(AppChange::Visibility),
        Event::MenuSource {
            kind,
            bus,
            app_path,
            menubar_path,
        } => vocab(kind)
            .map(|kind| AppChange::Menu(MenuSource::new(kind, bus, app_path, menubar_path))),
        Event::Attention { level } => vocab(level).map(AppChange::Attention),
        Event::Mru { rank } => Some(AppChange::Mru(rank)),
        Event::Done => {
            emit(wire, QuireEvent::App(AppEvent::Done(key)));
            None
        }
        Event::Closed => {
            emit(wire, QuireEvent::App(AppEvent::Closed(key)));
            proxy.destroy();
            wire.handles.apps.remove(&key);
            None
        }
        _ => None,
    };
    app_change(wire, proxy, change);
}

fn window_change(
    wire: &mut Wire,
    object: WindowObject,
    change: Option<WindowChange>,
) -> Vec<QuireEvent> {
    match change {
        Some(change) => wire.linker.change(object, change),
        None => Vec::new(),
    }
}

/// A window object's events, through the linker.
pub(super) fn on_window(
    wire: &mut Wire,
    proxy: &QuireDesktopWindowV1,
    event: quire_desktop_window_v1::Event,
) {
    use quire_desktop_window_v1::Event;
    let object = WindowObject::new(number_of(proxy));
    let events = match event {
        Event::Identifier { identifier } => wire
            .linker
            .identified(object, ToplevelIdentifier::new(identifier)),
        Event::App { app } => {
            let app = AppKey::new(number_of(&app));
            window_change(wire, object, Some(WindowChange::App(app)))
        }
        Event::Role { role, modality } => {
            let change = vocab(role)
                .zip(vocab(modality))
                .map(|(role, modality)| WindowChange::Role { role, modality });
            window_change(wire, object, change)
        }
        Event::State { kind, actor } => {
            let change = vocab(kind).map(|kind| WindowChange::State {
                kind,
                actor: vocab(actor).unwrap_or(ActorKind::Unknown),
            });
            window_change(wire, object, change)
        }
        Event::Place { kind, space } => {
            let place = match (vocab(kind), space) {
                (Some(PlaceKind::OnSpace), Some(space)) => {
                    Some(SpacePlace::On(SpaceKey::new(number_of(&space))))
                }
                (Some(PlaceKind::AllSpaces), _) => Some(SpacePlace::AllSpaces),
                _ => None,
            };
            window_change(wire, object, place.map(WindowChange::Place))
        }
        Event::Output { output } => {
            let name = wire.output_name(&output);
            window_change(wire, object, name.map(WindowChange::Output))
        }
        Event::Geometry {
            x,
            y,
            width,
            height,
            scale120,
        } => {
            let geometry = WindowGeometry::new(Rect::new(x, y, width, height), scale120);
            window_change(wire, object, Some(WindowChange::Geometry(geometry)))
        }
        Event::Parent { parent } => {
            let parent = parent
                .as_ref()
                .and_then(|p| wire.linker.key_of(WindowObject::new(number_of(p))));
            window_change(wire, object, Some(WindowChange::Parent(parent)))
        }
        Event::Emphasis { emphasis } => {
            window_change(wire, object, vocab(emphasis).map(WindowChange::Emphasis))
        }
        Event::Sensitivity { level } => {
            window_change(wire, object, vocab(level).map(WindowChange::Sensitivity))
        }
        Event::Lease { state, glow } => {
            let change = vocab(state).map(|state| {
                WindowChange::Lease(Lease::new(state, vocab(glow).unwrap_or(GlowCode::Off)))
            });
            window_change(wire, object, change)
        }
        Event::Responsiveness { state } => {
            window_change(wire, object, vocab(state).map(WindowChange::Responsiveness))
        }
        Event::Placement { mark } => {
            window_change(wire, object, vocab(mark).map(WindowChange::Placement))
        }
        Event::MenuPath { path } => window_change(wire, object, Some(WindowChange::MenuPath(path))),
        Event::Done => wire.linker.done(object),
        Event::Closed => {
            let events = wire.linker.closed(object);
            proxy.destroy();
            wire.handles.windows.remove(&object);
            events
        }
        _ => Vec::new(),
    };
    emit_all(wire, events);
}

/// A Space's events.
pub(super) fn on_space(
    wire: &mut Wire,
    proxy: &QuireDesktopSpaceV1,
    event: quire_desktop_space_v1::Event,
) {
    use quire_desktop_space_v1::Event;
    let key = SpaceKey::new(number_of(proxy));
    let change = match event {
        Event::Id { space_id } => Some(SpaceChange::Id(SpaceId::new(space_id))),
        Event::Kind { kind } => vocab(kind).map(SpaceChange::Kind),
        Event::Occupant { token } => Some(SpaceChange::Occupant(token)),
        Event::Position { index } => Some(SpaceChange::Position(index)),
        Event::Output { output } => wire.output_name(&output).map(SpaceChange::Output),
        Event::Activity { activity } => vocab(activity).map(SpaceChange::Activity),
        Event::Done => {
            emit(wire, QuireEvent::Space(SpaceEvent::Done(key)));
            None
        }
        Event::Removed => {
            emit(wire, QuireEvent::Space(SpaceEvent::Closed(key)));
            proxy.destroy();
            wire.handles.spaces.remove(&key);
            None
        }
        _ => None,
    };
    if let Some(change) = change {
        emit(wire, QuireEvent::Space(SpaceEvent::Changed { key, change }));
    }
}

impl Dispatch<QuireDesktopManagerV1, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &QuireDesktopManagerV1,
        event: quire_desktop_manager_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_manager(state, proxy, event);
    }

    event_created_child!(Wire, QuireDesktopManagerV1, [
        quire_desktop_manager_v1::EVT_APP_OPCODE => (QuireDesktopAppV1, ()),
        quire_desktop_manager_v1::EVT_WINDOW_OPCODE => (QuireDesktopWindowV1, ()),
        quire_desktop_manager_v1::EVT_SPACE_OPCODE => (QuireDesktopSpaceV1, ()),
    ]);
}

impl Dispatch<QuireDesktopAppV1, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &QuireDesktopAppV1,
        event: quire_desktop_app_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_app(state, proxy, event);
    }
}

impl Dispatch<QuireDesktopWindowV1, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &QuireDesktopWindowV1,
        event: quire_desktop_window_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_window(state, proxy, event);
    }
}

impl Dispatch<QuireDesktopSpaceV1, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &QuireDesktopSpaceV1,
        event: quire_desktop_space_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_space(state, proxy, event);
    }
}
