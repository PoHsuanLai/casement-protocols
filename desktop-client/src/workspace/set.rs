//! The workspaces known now.

use std::collections::BTreeMap;

use super::flags::WorkspaceFlag;
use super::model::{Group, Workspace};
use crate::keys::{GroupKey, WorkspaceKey};

/// Workspaces and groups at one moment.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snapshot {
    pub(crate) workspaces: BTreeMap<WorkspaceKey, Workspace>,
    pub(crate) groups: BTreeMap<GroupKey, Group>,
}

/// The committed workspaces readers see, and the pending ones events edit.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Workspaces {
    pub(crate) committed: Snapshot,
    pub(crate) pending: Snapshot,
}

impl Workspaces {
    /// The committed workspaces, in creation order.
    pub fn workspaces(&self) -> impl Iterator<Item = &Workspace> {
        self.committed.workspaces.values()
    }

    /// The committed groups, in creation order.
    pub fn groups(&self) -> impl Iterator<Item = &Group> {
        self.committed.groups.values()
    }

    /// One committed workspace.
    pub fn get(&self, key: WorkspaceKey) -> Option<&Workspace> {
        self.committed.workspaces.get(&key)
    }

    /// The committed active workspaces (one per group in the usual case).
    pub fn active(&self) -> impl Iterator<Item = &Workspace> {
        self.workspaces()
            .filter(|w| w.flags.contains(&WorkspaceFlag::Active))
    }
}
