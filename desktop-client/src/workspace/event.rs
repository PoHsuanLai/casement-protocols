//! What `ext-workspace-v1` reports.

use super::flags::{GroupAbility, Set, WorkspaceAbility, WorkspaceFlag};
use crate::keys::{GroupKey, WorkspaceKey};
use crate::names::OutputName;

/// One property of a workspace changing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WorkspaceChange {
    /// The id.
    Id(String),
    /// The name.
    Name(String),
    /// The coordinates.
    Coordinates(Vec<u32>),
    /// The whole state set.
    Flags(Set<WorkspaceFlag>),
    /// The whole ability set.
    Abilities(Set<WorkspaceAbility>),
    /// Joined this group.
    GroupEntered(GroupKey),
    /// Left this group.
    GroupLeft(GroupKey),
}

/// One property of a group changing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GroupChange {
    /// The whole ability set.
    Abilities(Set<GroupAbility>),
    /// Now on this output.
    OutputEntered(OutputName),
    /// No longer on this output.
    OutputLeft(OutputName),
}

/// A workspace event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WorkspaceEvent {
    /// A group appeared.
    GroupOpened(GroupKey),
    /// A group changed.
    Group {
        /// The group.
        key: GroupKey,
        /// What changed.
        change: GroupChange,
    },
    /// A group is gone.
    GroupRemoved(GroupKey),
    /// A workspace appeared.
    Opened(WorkspaceKey),
    /// A workspace changed.
    Changed {
        /// The workspace.
        key: WorkspaceKey,
        /// What changed.
        change: WorkspaceChange,
    },
    /// A workspace is gone.
    Removed(WorkspaceKey),
    /// The batch is complete: pending becomes committed (`done`).
    Done,
    /// The manager is finished; no more events (`finished`).
    Finished,
}
