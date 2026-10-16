//! The display trial: a program changed the displays and the person decides whether to keep
//! the new setup (`quire_shell_v1`'s `displays_trial`, `displays_trial_end` and
//! `displays_trial_answer`).

use super::code::wire_enum;

wire_enum! {
    /// What the person said about a changed display setup (`quire_shell_v1.trial_answer`).
    TrialAnswer {
        /// Keep: the new setup stays.
        Keep = 0,
        /// Revert: the compositor goes back to the previous setup.
        Revert = 1,
    }
}

/// The id of one display trial (`displays_trial`'s `trial`). A restarted trial may carry the
/// same id again.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct TrialId(u32);

impl TrialId {
    /// The id the wire carries as `value`.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// The number on the wire.
    pub const fn get(self) -> u32 {
        self.0
    }
}
