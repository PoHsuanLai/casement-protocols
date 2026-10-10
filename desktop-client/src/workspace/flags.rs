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

impl WorkspaceFlag {
    /// The states set in a `state` bitfield; unknown bits are skipped.
    pub fn set_from_bits(bits: u32) -> Set<Self> {
        [(1, Self::Active), (2, Self::Urgent), (4, Self::Hidden)]
            .into_iter()
            .filter(|(bit, _)| bits & bit != 0)
            .map(|(_, flag)| flag)
            .collect()
    }
}

impl WorkspaceAbility {
    /// The abilities set in a `workspace_capabilities` bitfield; unknown bits are skipped.
    pub fn set_from_bits(bits: u32) -> Set<Self> {
        [
            (1, Self::Activate),
            (2, Self::Deactivate),
            (4, Self::Remove),
            (8, Self::Assign),
        ]
        .into_iter()
        .filter(|(bit, _)| bits & bit != 0)
        .map(|(_, ability)| ability)
        .collect()
    }
}

impl GroupAbility {
    /// The abilities set in a `group_capabilities` bitfield; unknown bits are skipped.
    pub fn set_from_bits(bits: u32) -> Set<Self> {
        [(1, Self::CreateWorkspace)]
            .into_iter()
            .filter(|(bit, _)| bits & bit != 0)
            .map(|(_, ability)| ability)
            .collect()
    }
}

/// The coordinates in a wire `coordinates` array (native-endian `u32`s); a trailing partial
/// word is skipped.
pub fn coordinates_from_array(bytes: &[u8]) -> Vec<u32> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|w| u32::from_ne_bytes(*w))
        .collect()
}
