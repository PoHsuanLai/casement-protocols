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

impl ToplevelFlag {
    /// The state with this `zwlr_foreign_toplevel_handle_v1.state` code; `None` for a code
    /// this crate does not know.
    pub const fn from_code(code: u32) -> Option<Self> {
        match code {
            0 => Some(Self::Maximized),
            1 => Some(Self::Minimized),
            2 => Some(Self::Activated),
            3 => Some(Self::Fullscreen),
            _ => None,
        }
    }
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

    /// The states in a wire `state` array (native-endian `u32` codes); unknown codes and a
    /// trailing partial word are skipped.
    pub fn from_array(bytes: &[u8]) -> Self {
        Self::of(
            bytes
                .as_chunks::<4>()
                .0
                .iter()
                .filter_map(|w| ToplevelFlag::from_code(u32::from_ne_bytes(*w))),
        )
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
