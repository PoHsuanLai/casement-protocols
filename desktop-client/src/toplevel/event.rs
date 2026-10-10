//! What the toplevel protocols report.

use super::flags::ToplevelFlags;
use crate::keys::ToplevelKey;
use crate::names::{OutputName, ToplevelIdentifier};

/// One property of a toplevel changing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ToplevelChange {
    /// The stable identifier (`ext_foreign_toplevel_handle_v1.identifier`).
    Identifier(ToplevelIdentifier),
    /// The title.
    Title(String),
    /// The app id.
    AppId(String),
    /// Now on this output.
    OutputEntered(OutputName),
    /// No longer on this output.
    OutputLeft(OutputName),
    /// The whole state set.
    Flags(ToplevelFlags),
    /// The parent, or none.
    Parent(Option<ToplevelKey>),
}

/// A toplevel event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ToplevelEvent {
    /// A handle appeared.
    Opened(ToplevelKey),
    /// A property changed; takes effect for readers at the next `Done`.
    Changed {
        /// The toplevel.
        key: ToplevelKey,
        /// What changed.
        change: ToplevelChange,
    },
    /// The compositor finished a batch of changes (`done`).
    Done(ToplevelKey),
    /// `from` and `into` are one window: `from`'s identifier moves onto `into`, `from` goes.
    Merged {
        /// The record that held the identifier (from the list).
        from: ToplevelKey,
        /// The record that holds the state (from management).
        into: ToplevelKey,
    },
    /// The toplevel is gone (`closed`).
    Closed(ToplevelKey),
}
