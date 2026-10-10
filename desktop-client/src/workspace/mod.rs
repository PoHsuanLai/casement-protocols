//! Workspaces: `ext-workspace-v1`.
//!
//! The protocol applies changes atomically at `done`, so [`Workspaces`] keeps a pending copy
//! that events edit and a committed copy readers see.

mod event;
mod flags;
mod model;
mod reduce;
mod set;

pub use event::{GroupChange, WorkspaceChange, WorkspaceEvent};
pub use flags::{GroupAbility, WorkspaceAbility, WorkspaceFlag};
pub use model::{Group, Workspace};
pub(crate) use reduce::reduce_workspaces;
pub use set::{Snapshot, Workspaces};
