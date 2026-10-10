//! The workspace transition.

use super::event::{GroupChange, WorkspaceChange, WorkspaceEvent};
use super::model::{Group, Workspace};
use super::set::Workspaces;

/// `workspaces` after `event`: every event but `Done` edits the pending snapshot; `Done`
/// copies it to the committed one. A change to an unknown key is ignored. `Finished` changes
/// nothing: the last committed picture stays readable.
pub(crate) fn reduce_workspaces(mut workspaces: Workspaces, event: WorkspaceEvent) -> Workspaces {
    match event {
        WorkspaceEvent::Done => workspaces.committed = workspaces.pending.clone(),
        WorkspaceEvent::Finished => {}
        WorkspaceEvent::GroupOpened(key) => {
            workspaces
                .pending
                .groups
                .entry(key)
                .or_insert_with(|| Group::new(key));
        }
        WorkspaceEvent::Group { key, change } => {
            if let Some(group) = workspaces.pending.groups.get_mut(&key) {
                apply_group(group, change);
            }
        }
        WorkspaceEvent::GroupRemoved(key) => {
            workspaces.pending.groups.remove(&key);
            for workspace in workspaces.pending.workspaces.values_mut() {
                if workspace.group == Some(key) {
                    workspace.group = None;
                }
            }
        }
        WorkspaceEvent::Opened(key) => {
            workspaces
                .pending
                .workspaces
                .entry(key)
                .or_insert_with(|| Workspace::new(key));
        }
        WorkspaceEvent::Changed { key, change } => {
            if let Some(workspace) = workspaces.pending.workspaces.get_mut(&key) {
                apply_workspace(workspace, change);
            }
        }
        WorkspaceEvent::Removed(key) => {
            workspaces.pending.workspaces.remove(&key);
        }
    }
    workspaces
}

fn apply_group(group: &mut Group, change: GroupChange) {
    match change {
        GroupChange::Abilities(abilities) => group.abilities = abilities,
        GroupChange::OutputEntered(name) => {
            if !group.outputs.contains(&name) {
                group.outputs.push(name);
            }
        }
        GroupChange::OutputLeft(name) => group.outputs.retain(|o| o != &name),
    }
}

fn apply_workspace(workspace: &mut Workspace, change: WorkspaceChange) {
    match change {
        WorkspaceChange::Id(id) => workspace.id = Some(id),
        WorkspaceChange::Name(name) => workspace.name = name,
        WorkspaceChange::Coordinates(coordinates) => workspace.coordinates = coordinates,
        WorkspaceChange::Flags(flags) => workspace.flags = flags,
        WorkspaceChange::Abilities(abilities) => workspace.abilities = abilities,
        WorkspaceChange::GroupEntered(group) => workspace.group = Some(group),
        WorkspaceChange::GroupLeft(group) => {
            if workspace.group == Some(group) {
                workspace.group = None;
            }
        }
    }
}
