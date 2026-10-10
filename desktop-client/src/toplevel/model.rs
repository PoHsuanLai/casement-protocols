//! One toplevel as far as the compositor has said.

use super::flags::ToplevelFlags;
use crate::keys::ToplevelKey;
use crate::names::{OutputName, ToplevelIdentifier};

/// Whether the compositor has finished describing a toplevel (its first `done`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Announcement {
    /// Before the first `done`: half described, not to be shown.
    #[default]
    Pending,
    /// Complete.
    Announced,
}

/// A toplevel window.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Toplevel {
    /// This client's key for it.
    pub key: ToplevelKey,
    /// The compositor's stable identifier; `None` without the toplevel list or before pairing.
    pub identifier: Option<ToplevelIdentifier>,
    /// The title.
    pub title: String,
    /// The app id the client set; empty when none.
    pub app_id: String,
    /// Maximised, minimised, activated, full screen.
    pub flags: ToplevelFlags,
    /// The outputs it is shown on.
    pub outputs: Vec<OutputName>,
    /// Its parent (a dialog's window), when the compositor says.
    pub parent: Option<ToplevelKey>,
    /// Whether it is complete.
    pub announcement: Announcement,
}

impl Toplevel {
    /// A toplevel that has just appeared: nothing known yet.
    pub fn new(key: ToplevelKey) -> Self {
        Self {
            key,
            identifier: None,
            title: String::new(),
            app_id: String::new(),
            flags: ToplevelFlags::none(),
            outputs: Vec::new(),
            parent: None,
            announcement: Announcement::Pending,
        }
    }
}
