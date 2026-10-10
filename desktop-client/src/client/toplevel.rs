//! Toplevel dispatch: the list, the manager and their handles.

use wayland_protocols::ext::foreign_toplevel_list::v1::client::{
    ext_foreign_toplevel_handle_v1, ext_foreign_toplevel_list_v1,
};
use wayland_protocols_wlr::foreign_toplevel::v1::client::{
    zwlr_foreign_toplevel_handle_v1, zwlr_foreign_toplevel_manager_v1,
};

use super::wire::Wire;
use crate::keys::ToplevelKey;

// The fill lane adds, in this file only, the `Dispatch` impls for the four interfaces:
//   ExtForeignToplevelListV1 (event_created_child for `toplevel`), user data `()`;
//   ExtForeignToplevelHandleV1, user data `ToplevelKey`;
//   ZwlrForeignToplevelManagerV1 (event_created_child for `toplevel`), user data `()`;
//   ZwlrForeignToplevelHandleV1, user data `ToplevelKey`.
// Each calls one of the functions below, which turn an event into `ToplevelEvent`s through
// `Wire::emit` and pair list and managed handles with `crate::toplevel::pair` at each `done`.
// The wlr `state` array maps to `ToplevelFlags`; `output_enter` resolves the `wl_output` to
// the name it announced.

/// A wlr handle's event.
pub(super) fn on_managed(
    wire: &mut Wire,
    key: ToplevelKey,
    event: zwlr_foreign_toplevel_handle_v1::Event,
) {
    let _ = (wire, key, event);
    todo!("fill")
}

/// A list handle's event.
pub(super) fn on_listed(
    wire: &mut Wire,
    key: ToplevelKey,
    event: ext_foreign_toplevel_handle_v1::Event,
) {
    let _ = (wire, key, event);
    todo!("fill")
}

/// The list's event: a new handle, or `finished`.
pub(super) fn on_list(wire: &mut Wire, event: ext_foreign_toplevel_list_v1::Event) {
    let _ = (wire, event);
    todo!("fill")
}

/// The manager's event: a new handle, or `finished`.
pub(super) fn on_manager(wire: &mut Wire, event: zwlr_foreign_toplevel_manager_v1::Event) {
    let _ = (wire, event);
    todo!("fill")
}
