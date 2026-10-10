//! The states a toplevel can be in at once (`zwlr_foreign_toplevel_handle_v1.state`).

use std::collections::BTreeSet;

/// One state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ToplevelFlag {
    /// Maximised.
    Maximized,
    /// Minimised.
    Minimized,
    /// Has the keyboard focus (the seat's active window).
    Activated,
    /// Full screen.
    Fullscreen,
}

/// The set of states a toplevel is in.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ToplevelFlags(BTreeSet<ToplevelFlag>);

impl ToplevelFlags {
    /// No state set.
    pub fn none() -> Self {
        Self::default()
    }

    /// Exactly these states.
    pub fn of(flags: impl IntoIterator<Item = ToplevelFlag>) -> Self {
        Self(flags.into_iter().collect())
    }

    /// Whether `flag` is set.
    pub fn contains(&self, flag: ToplevelFlag) -> bool {
        self.0.contains(&flag)
    }

    /// The states, in order.
    pub fn iter(&self) -> impl Iterator<Item = ToplevelFlag> + '_ {
        self.0.iter().copied()
    }
}
