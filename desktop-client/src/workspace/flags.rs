//! Workspace state and abilities.

use std::collections::BTreeSet;

/// A workspace state bit (`ext_workspace_handle_v1.state`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum WorkspaceFlag {
    /// The workspace on show.
    Active,
    /// Wants attention.
    Urgent,
    /// Not to be listed.
    Hidden,
}

/// What a workspace lets the client request (`ext_workspace_handle_v1.workspace_capabilities`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum WorkspaceAbility {
    /// `activate`.
    Activate,
    /// `deactivate`.
    Deactivate,
    /// `remove`.
    Remove,
    /// `assign` to another group.
    Assign,
}

/// What a group lets the client request (`group_capabilities`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum GroupAbility {
    /// `create_workspace`.
    CreateWorkspace,
}

/// A set of values of an ordered kind; one type for flags and abilities.
pub(crate) type Set<T> = BTreeSet<T>;
