//! A question from the compositor to the person (`quire_shell_v1`, version 6): a program
//! changed the displays; keep the new setup?

use std::time::Duration;

use casement_protocols::vocab::TrialId;

/// One trial, as `displays_trial` carries it.
///
/// The shell puts "Keep these display settings?" on screen with Keep and Revert and a countdown,
/// and answers with [`DesktopCommand::AnswerTrial`](crate::DesktopCommand::AnswerTrial). The
/// compositor reverts by itself when the time runs out. A further change while a trial is open
/// sends this again, possibly with the same [`TrialId`], so a shell treats an id it already shows
/// as the same question with a restarted countdown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct DisplayTrial {
    /// The trial's id; the answer and the end name it.
    pub trial: TrialId,
    /// How long the person has, in seconds.
    pub seconds: u32,
}

impl DisplayTrial {
    /// A trial of `seconds` seconds.
    pub const fn new(trial: TrialId, seconds: u32) -> Self {
        Self { trial, seconds }
    }

    /// How long the person has.
    pub fn within(&self) -> Duration {
        Duration::from_secs(u64::from(self.seconds))
    }
}
