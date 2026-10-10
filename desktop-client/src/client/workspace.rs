//! Workspace dispatch: the manager, groups and workspaces (`ext-workspace-v1`).

use wayland_protocols::ext::workspace::v1::client::{
    ext_workspace_group_handle_v1, ext_workspace_handle_v1, ext_workspace_manager_v1,
};

use super::wire::Wire;
use crate::keys::{GroupKey, WorkspaceKey};

// The fill lane adds the `Dispatch` impls (manager `()`, group `GroupKey`, workspace
// `WorkspaceKey`; event_created_child for `workspace_group` and `workspace`). The state
// bitfield maps to `WorkspaceFlag`s, the capabilities bitfields to the ability sets, and the
// coordinates array of native-endian u32 to `Vec<u32>`.

/// The manager's event.
pub(super) fn on_manager(wire: &mut Wire, event: ext_workspace_manager_v1::Event) {
    let _ = (wire, event);
    todo!("fill")
}

/// A group's event.
pub(super) fn on_group(
    wire: &mut Wire,
    key: GroupKey,
    event: ext_workspace_group_handle_v1::Event,
) {
    let _ = (wire, key, event);
    todo!("fill")
}

/// A workspace's event.
pub(super) fn on_workspace(
    wire: &mut Wire,
    key: WorkspaceKey,
    event: ext_workspace_handle_v1::Event,
) {
    let _ = (wire, key, event);
    todo!("fill")
}
