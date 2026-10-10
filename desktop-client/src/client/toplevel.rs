//! Toplevel dispatch: the list, the manager and their handles.
//!
//! A window has two handles, one per protocol (FINDINGS). A management handle becomes visible
//! at its first `done`. When the manager is offered, a list handle is held back until
//! `toplevel::pair` matches it with a management handle, and then merged into it; without the
//! manager it is announced on its own `done`.

use wayland_client::{Connection, Dispatch, QueueHandle, event_created_child};
use wayland_protocols::ext::foreign_toplevel_list::v1::client::{
    ext_foreign_toplevel_handle_v1::{self, ExtForeignToplevelHandleV1},
    ext_foreign_toplevel_list_v1::{self, ExtForeignToplevelListV1},
};
use wayland_protocols_wlr::foreign_toplevel::v1::client::{
    zwlr_foreign_toplevel_handle_v1::{self, ZwlrForeignToplevelHandleV1},
    zwlr_foreign_toplevel_manager_v1::{self, ZwlrForeignToplevelManagerV1},
};

use super::wire::{Wire, number_of};
use crate::event::DesktopEvent;
use crate::keys::ToplevelKey;
use crate::names::ToplevelIdentifier;
use crate::toplevel::{ToplevelChange, ToplevelEvent, ToplevelFlags};

fn toplevel_event(wire: &mut Wire, event: ToplevelEvent) {
    wire.emit(DesktopEvent::Toplevel(event));
}

fn change(wire: &mut Wire, key: ToplevelKey, change: ToplevelChange) {
    toplevel_event(wire, ToplevelEvent::Changed { key, change });
}

/// Matches what waits and emits the merges.
fn pair_up(wire: &mut Wire) {
    for event in wire.book.pair_up(&wire.state) {
        wire.emit(event);
    }
}

/// A wlr handle's event.
pub(super) fn on_managed(
    wire: &mut Wire,
    key: ToplevelKey,
    event: zwlr_foreign_toplevel_handle_v1::Event,
) {
    use zwlr_foreign_toplevel_handle_v1::Event;
    match event {
        Event::Title { title } => change(wire, key, ToplevelChange::Title(title)),
        Event::AppId { app_id } => change(wire, key, ToplevelChange::AppId(app_id)),
        Event::OutputEnter { output } => {
            if let Some(name) = wire.output_name(&output) {
                change(wire, key, ToplevelChange::OutputEntered(name));
            }
        }
        Event::OutputLeave { output } => {
            if let Some(name) = wire.output_name(&output) {
                change(wire, key, ToplevelChange::OutputLeft(name));
            }
        }
        Event::State { state } => {
            change(
                wire,
                key,
                ToplevelChange::Flags(ToplevelFlags::from_array(&state)),
            );
        }
        Event::Parent { parent } => {
            let parent = parent.as_ref().map(|p| ToplevelKey::new(number_of(p)));
            change(wire, key, ToplevelChange::Parent(parent));
        }
        Event::Done => {
            toplevel_event(wire, ToplevelEvent::Done(key));
            if wire.handles.toplevel_list.is_some() {
                wire.book.add_managed(key);
                pair_up(wire);
            }
        }
        Event::Closed => {
            toplevel_event(wire, ToplevelEvent::Closed(key));
            if let Some(handle) = wire.handles.managed.remove(&key) {
                handle.destroy();
            }
            if let Some(listed) = wire.book.drop_managed(key)
                && let Some(handle) = wire.handles.listed.remove(&listed)
            {
                handle.destroy();
            }
        }
        _ => {}
    }
}

/// A list handle's event.
pub(super) fn on_listed(
    wire: &mut Wire,
    key: ToplevelKey,
    event: ext_foreign_toplevel_handle_v1::Event,
) {
    use ext_foreign_toplevel_handle_v1::Event;
    if wire.book.is_merged(key) {
        return;
    }
    match event {
        Event::Title { title } => change(wire, key, ToplevelChange::Title(title)),
        Event::AppId { app_id } => change(wire, key, ToplevelChange::AppId(app_id)),
        Event::Identifier { identifier } => {
            let identifier = ToplevelIdentifier::new(identifier);
            change(wire, key, ToplevelChange::Identifier(identifier.clone()));
            // On the Quire arm the window object of the same identifier may be waiting.
            for event in wire.linker.listed(key, identifier) {
                wire.emit(DesktopEvent::Quire(event));
            }
        }
        Event::Done => {
            if wire.handles.toplevel_manager.is_some() {
                wire.book.add_listed(key);
                pair_up(wire);
            } else {
                toplevel_event(wire, ToplevelEvent::Done(key));
            }
        }
        Event::Closed => {
            wire.book.drop_listed(key);
            wire.linker.unlisted(key);
            toplevel_event(wire, ToplevelEvent::Closed(key));
            if let Some(handle) = wire.handles.listed.remove(&key) {
                handle.destroy();
            }
        }
        _ => {}
    }
}

/// The list's event: a new handle, or `finished`.
pub(super) fn on_list(wire: &mut Wire, event: ext_foreign_toplevel_list_v1::Event) {
    if let ext_foreign_toplevel_list_v1::Event::Toplevel { toplevel } = event {
        let key = ToplevelKey::new(number_of(&toplevel));
        wire.handles.listed.insert(key, toplevel);
        toplevel_event(wire, ToplevelEvent::Opened(key));
    }
}

/// The manager's event: a new handle, or `finished`.
pub(super) fn on_manager(wire: &mut Wire, event: zwlr_foreign_toplevel_manager_v1::Event) {
    if let zwlr_foreign_toplevel_manager_v1::Event::Toplevel { toplevel } = event {
        let key = ToplevelKey::new(number_of(&toplevel));
        wire.handles.managed.insert(key, toplevel);
        toplevel_event(wire, ToplevelEvent::Opened(key));
    }
}

impl Dispatch<ExtForeignToplevelListV1, ()> for Wire {
    fn event(
        state: &mut Self,
        _proxy: &ExtForeignToplevelListV1,
        event: ext_foreign_toplevel_list_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_list(state, event);
    }

    event_created_child!(Wire, ExtForeignToplevelListV1, [
        ext_foreign_toplevel_list_v1::EVT_TOPLEVEL_OPCODE => (ExtForeignToplevelHandleV1, ()),
    ]);
}

impl Dispatch<ExtForeignToplevelHandleV1, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &ExtForeignToplevelHandleV1,
        event: ext_foreign_toplevel_handle_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_listed(state, ToplevelKey::new(number_of(proxy)), event);
    }
}

impl Dispatch<ZwlrForeignToplevelManagerV1, ()> for Wire {
    fn event(
        state: &mut Self,
        _proxy: &ZwlrForeignToplevelManagerV1,
        event: zwlr_foreign_toplevel_manager_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_manager(state, event);
    }

    event_created_child!(Wire, ZwlrForeignToplevelManagerV1, [
        zwlr_foreign_toplevel_manager_v1::EVT_TOPLEVEL_OPCODE => (ZwlrForeignToplevelHandleV1, ()),
    ]);
}

impl Dispatch<ZwlrForeignToplevelHandleV1, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &ZwlrForeignToplevelHandleV1,
        event: zwlr_foreign_toplevel_handle_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_managed(state, ToplevelKey::new(number_of(proxy)), event);
    }
}
