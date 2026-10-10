//! A workspace and a group.

use super::flags::{GroupAbility, Set, WorkspaceAbility, WorkspaceFlag};
use crate::keys::{GroupKey, WorkspaceKey};
use crate::names::OutputName;

/// One workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Workspace {
    /// This client's key for it.
    pub key: WorkspaceKey,
    /// The compositor's id, stable across sessions when it gives one.
    pub id: Option<String>,
    /// The name a person reads.
    pub name: String,
    /// Where it sits in the compositor's arrangement (one number per dimension); empty when
    /// the compositor gives none.
    pub coordinates: Vec<u32>,
    /// Active, urgent, hidden.
    pub flags: Set<WorkspaceFlag>,
    /// What the client may ask of it.
    pub abilities: Set<WorkspaceAbility>,
    /// The group it belongs to.
    pub group: Option<GroupKey>,
}

impl Workspace {
    /// A workspace that has just appeared.
    pub fn new(key: WorkspaceKey) -> Self {
        Self {
            key,
            id: None,
            name: String::new(),
            coordinates: Vec::new(),
            flags: Set::new(),
            abilities: Set::new(),
            group: None,
        }
    }
}

/// A set of workspaces shown together on some outputs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Group {
    /// This client's key for it.
    pub key: GroupKey,
    /// The outputs it spans.
    pub outputs: Vec<OutputName>,
    /// What the client may ask of it.
    pub abilities: Set<GroupAbility>,
}

impl Group {
    /// A group that has just appeared.
    pub fn new(key: GroupKey) -> Self {
        Self {
            key,
            outputs: Vec::new(),
            abilities: Set::new(),
        }
    }
}
