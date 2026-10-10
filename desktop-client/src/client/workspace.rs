//! Workspace dispatch: the manager, groups and workspaces (`ext-workspace-v1`).

use wayland_client::{Connection, Dispatch, QueueHandle, event_created_child};
use wayland_protocols::ext::workspace::v1::client::{
    ext_workspace_group_handle_v1::{self, ExtWorkspaceGroupHandleV1},
    ext_workspace_handle_v1::{self, ExtWorkspaceHandleV1},
    ext_workspace_manager_v1::{self, ExtWorkspaceManagerV1},
};

use super::wire::{Wire, number_of, raw};
use crate::event::DesktopEvent;
use crate::keys::{GroupKey, WorkspaceKey};
use crate::workspace::{
    GroupAbility, GroupChange, WorkspaceAbility, WorkspaceChange, WorkspaceEvent, WorkspaceFlag,
    coordinates_from_array,
};

fn workspace_event(wire: &mut Wire, event: WorkspaceEvent) {
    wire.emit(DesktopEvent::Workspace(event));
}

fn workspace_change(wire: &mut Wire, key: WorkspaceKey, change: WorkspaceChange) {
    workspace_event(wire, WorkspaceEvent::Changed { key, change });
}

fn group_change(wire: &mut Wire, key: GroupKey, change: GroupChange) {
    workspace_event(wire, WorkspaceEvent::Group { key, change });
}

/// The manager's event.
pub(super) fn on_manager(wire: &mut Wire, event: ext_workspace_manager_v1::Event) {
    use ext_workspace_manager_v1::Event;
    match event {
        Event::WorkspaceGroup { workspace_group } => {
            let key = GroupKey::new(number_of(&workspace_group));
            wire.handles.groups.insert(key, workspace_group);
            workspace_event(wire, WorkspaceEvent::GroupOpened(key));
        }
        Event::Workspace { workspace } => {
            let key = WorkspaceKey::new(number_of(&workspace));
            wire.handles.workspaces.insert(key, workspace);
            workspace_event(wire, WorkspaceEvent::Opened(key));
        }
        Event::Done => workspace_event(wire, WorkspaceEvent::Done),
        Event::Finished => workspace_event(wire, WorkspaceEvent::Finished),
        _ => {}
    }
}

/// A group's event.
pub(super) fn on_group(
    wire: &mut Wire,
    key: GroupKey,
    event: ext_workspace_group_handle_v1::Event,
) {
    use ext_workspace_group_handle_v1::Event;
    match event {
        Event::Capabilities { capabilities } => {
            let abilities = GroupAbility::set_from_bits(raw(capabilities));
            group_change(wire, key, GroupChange::Abilities(abilities));
        }
        Event::OutputEnter { output } => {
            if let Some(name) = wire.output_name(&output) {
                group_change(wire, key, GroupChange::OutputEntered(name));
            }
        }
        Event::OutputLeave { output } => {
            if let Some(name) = wire.output_name(&output) {
                group_change(wire, key, GroupChange::OutputLeft(name));
            }
        }
        Event::WorkspaceEnter { workspace } => {
            let workspace = WorkspaceKey::new(number_of(&workspace));
            workspace_change(wire, workspace, WorkspaceChange::GroupEntered(key));
        }
        Event::WorkspaceLeave { workspace } => {
            let workspace = WorkspaceKey::new(number_of(&workspace));
            workspace_change(wire, workspace, WorkspaceChange::GroupLeft(key));
        }
        Event::Removed => {
            workspace_event(wire, WorkspaceEvent::GroupRemoved(key));
            if let Some(handle) = wire.handles.groups.remove(&key) {
                handle.destroy();
            }
        }
        _ => {}
    }
}

/// A workspace's event.
pub(super) fn on_workspace(
    wire: &mut Wire,
    key: WorkspaceKey,
    event: ext_workspace_handle_v1::Event,
) {
    use ext_workspace_handle_v1::Event;
    match event {
        Event::Id { id } => workspace_change(wire, key, WorkspaceChange::Id(id)),
        Event::Name { name } => workspace_change(wire, key, WorkspaceChange::Name(name)),
        Event::Coordinates { coordinates } => {
            let coordinates = coordinates_from_array(&coordinates);
            workspace_change(wire, key, WorkspaceChange::Coordinates(coordinates));
        }
        Event::State { state } => {
            let flags = WorkspaceFlag::set_from_bits(raw(state));
            workspace_change(wire, key, WorkspaceChange::Flags(flags));
        }
        Event::Capabilities { capabilities } => {
            let abilities = WorkspaceAbility::set_from_bits(raw(capabilities));
            workspace_change(wire, key, WorkspaceChange::Abilities(abilities));
        }
        Event::Removed => {
            workspace_event(wire, WorkspaceEvent::Removed(key));
            if let Some(handle) = wire.handles.workspaces.remove(&key) {
                handle.destroy();
            }
        }
        _ => {}
    }
}

impl Dispatch<ExtWorkspaceManagerV1, ()> for Wire {
    fn event(
        state: &mut Self,
        _proxy: &ExtWorkspaceManagerV1,
        event: ext_workspace_manager_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_manager(state, event);
    }

    event_created_child!(Wire, ExtWorkspaceManagerV1, [
        ext_workspace_manager_v1::EVT_WORKSPACE_GROUP_OPCODE => (ExtWorkspaceGroupHandleV1, ()),
        ext_workspace_manager_v1::EVT_WORKSPACE_OPCODE => (ExtWorkspaceHandleV1, ()),
    ]);
}

impl Dispatch<ExtWorkspaceGroupHandleV1, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &ExtWorkspaceGroupHandleV1,
        event: ext_workspace_group_handle_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_group(state, GroupKey::new(number_of(proxy)), event);
    }
}

impl Dispatch<ExtWorkspaceHandleV1, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &ExtWorkspaceHandleV1,
        event: ext_workspace_handle_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_workspace(state, WorkspaceKey::new(number_of(proxy)), event);
    }
}
