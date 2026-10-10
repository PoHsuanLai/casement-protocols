//! Spaces: `quire_desktop_space_v1`.

use crate::keys::SpaceKey;
use crate::names::{OutputName, SpaceId};
use crate::toplevel::Announcement;
use casement_protocols::vocab::{SpaceActivity, SpaceKind};

/// One Space.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct QuireSpace {
    /// This client's key for it.
    pub key: SpaceKey,
    /// Its stable id, also its ext-workspace `id`; `None` before the `id` event.
    pub id: Option<SpaceId>,
    /// What it holds.
    pub kind: SpaceKind,
    /// The task token of an agent Space.
    pub occupant: Option<String>,
    /// Its position on its output, 0 the leftmost.
    pub position: u32,
    /// Its output.
    pub output: Option<OutputName>,
    /// What is happening on it.
    pub activity: SpaceActivity,
    /// Whether it is complete.
    pub announcement: Announcement,
}

impl QuireSpace {
    /// A Space that has just appeared: nothing known yet.
    pub fn new(key: SpaceKey) -> Self {
        Self {
            key,
            id: None,
            kind: SpaceKind::Desktop,
            occupant: None,
            position: 0,
            output: None,
            activity: SpaceActivity::None,
            announcement: Announcement::Pending,
        }
    }
}

/// One property of a Space changing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpaceChange {
    /// The Space's id.
    Id(SpaceId),
    /// What it holds.
    Kind(SpaceKind),
    /// The task token; empty for none.
    Occupant(String),
    /// Its position on its output.
    Position(u32),
    /// Its output.
    Output(OutputName),
    /// What is happening on it.
    Activity(SpaceActivity),
}

/// A Space event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpaceEvent {
    /// A Space appeared.
    Opened(SpaceKey),
    /// A property changed.
    Changed {
        /// The Space.
        key: SpaceKey,
        /// What changed.
        change: SpaceChange,
    },
    /// The Space's burst is complete.
    Done(SpaceKey),
    /// The Space is gone (`removed`).
    Removed(SpaceKey),
}

/// Applies `change` to `space`.
pub(super) fn apply(space: &mut QuireSpace, change: SpaceChange) {
    match change {
        SpaceChange::Id(id) => space.id = Some(id),
        SpaceChange::Kind(kind) => space.kind = kind,
        SpaceChange::Occupant(token) => {
            space.occupant = Some(token).filter(|t| !t.is_empty());
        }
        SpaceChange::Position(position) => space.position = position,
        SpaceChange::Output(name) => space.output = Some(name),
        SpaceChange::Activity(activity) => space.activity = activity,
    }
}
