//! The workspace transition.

use super::event::WorkspaceEvent;
use super::set::Workspaces;

/// `workspaces` after `event`: every event but `Done` edits the pending snapshot; `Done`
/// copies it to the committed one. A change to an unknown key is ignored.
pub(crate) fn reduce_workspaces(workspaces: Workspaces, event: WorkspaceEvent) -> Workspaces {
    let _ = (workspaces, event);
    todo!("fill: edit pending; Done commits; Removed and GroupRemoved drop the entry and its links")
}
