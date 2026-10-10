//! Toplevels: identity from `ext-foreign-toplevel-list-v1`, state from
//! `zwlr-foreign-toplevel-management-v1`.
//!
//! The two protocols give a window two handles that name no common id (FINDINGS). The wire
//! layer pairs them with [`pair`] and reports the result as one key; [`ToplevelEvent::Merged`]
//! folds the list-only record into the managed one.

mod event;
mod flags;
mod model;
mod pairing;
mod reduce;
mod set;

pub use event::{ToplevelChange, ToplevelEvent};
pub use flags::{ToplevelFlag, ToplevelFlags};
pub use model::{Announcement, Toplevel};
pub use pairing::{Listed, Pairing, pair};
pub(crate) use reduce::reduce_toplevels;
pub use set::Toplevels;
