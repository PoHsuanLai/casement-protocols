//! Binding the globals.

use wayland_client::QueueHandle;
use wayland_client::globals::GlobalList;

use super::error::WireError;
use super::wire::Wire;
use crate::protocol::ProtocolSet;

/// The offer in `globals`, as the protocols this crate uses.
pub(super) fn offered(globals: &GlobalList) -> ProtocolSet {
    globals
        .contents()
        .with_list(|list| ProtocolSet::from_interfaces(list.iter().map(|g| g.interface.as_str())))
}

/// Binds each offered protocol at the version this crate implements, and the first seat, the
/// shm and every `wl_output` (at version 4 for `name`).
pub(super) fn bind_all(
    wire: &mut Wire,
    globals: &GlobalList,
    qh: &QueueHandle<Wire>,
) -> Result<(), WireError> {
    let _ = (wire, globals, qh);
    todo!(
        "fill: GlobalList::bind with version ranges; ext_workspace v1, wlr toplevel 3, wlr output 4"
    )
}
