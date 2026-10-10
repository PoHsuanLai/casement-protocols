//! The fake itself.

use std::collections::VecDeque;

use crate::command::{DesktopCommand, Ticket};
use crate::desktop::Desktop;
use crate::error::CommandError;
use crate::event::DesktopEvent;
use crate::keys::{CaptureId, ConfigId};
use crate::protocol::ProtocolSet;
use crate::reduce::reduce;
use crate::state::DesktopState;
use crate::supports::missing;

/// A [`Desktop`] fed by a script.
///
/// Queue events with [`FakeDesktop::script`]; [`FakeDesktop::advance`] reduces the next one
/// into the state (as the wire client does when a compositor speaks). Commands are checked
/// with [`crate::supports`] like the real client does, then recorded.
#[derive(Debug)]
pub struct FakeDesktop {
    state: DesktopState,
    script: VecDeque<DesktopEvent>,
    applied: Vec<DesktopEvent>,
    sent: Vec<DesktopCommand>,
    next_id: u64,
}

impl FakeDesktop {
    /// A fake whose compositor offers `protocols`.
    pub fn new(protocols: ProtocolSet) -> Self {
        Self {
            state: DesktopState::new(protocols),
            script: VecDeque::new(),
            applied: Vec::new(),
            sent: Vec::new(),
            next_id: 1,
        }
    }

    /// Queues `event` to be reduced by a later [`FakeDesktop::advance`].
    pub fn script(&mut self, event: DesktopEvent) {
        self.script.push_back(event);
    }

    /// Reduces the next scripted event, if there is one.
    pub fn advance(&mut self) -> Advance {
        match self.script.pop_front() {
            Some(event) => {
                let state = std::mem::take(&mut self.state);
                self.state = reduce(state, event.clone());
                self.applied.push(event);
                Advance::Applied
            }
            None => Advance::Idle,
        }
    }

    /// Reduces every scripted event.
    pub fn advance_all(&mut self) {
        while self.advance() == Advance::Applied {}
    }

    /// The commands accepted so far, oldest first.
    pub fn sent(&self) -> &[DesktopCommand] {
        &self.sent
    }

    /// The commands accepted so far, emptying the record.
    pub fn take_sent(&mut self) -> Vec<DesktopCommand> {
        std::mem::take(&mut self.sent)
    }

    fn mint(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

/// Whether [`FakeDesktop::advance`] had anything to apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Advance {
    /// An event was reduced.
    Applied,
    /// The script is empty.
    Idle,
}

impl Desktop for FakeDesktop {
    type Error = CommandError;

    fn state(&self) -> &DesktopState {
        &self.state
    }

    fn send(&mut self, command: DesktopCommand) -> Result<Ticket, CommandError> {
        if let Some(needs) = missing(&self.state.protocols, &command) {
            return Err(CommandError::Unsupported {
                command: command.kind(),
                needs,
            });
        }
        let ticket = match &command {
            DesktopCommand::ApplyOutputs(_) | DesktopCommand::TestOutputs(_) => {
                Ticket::Config(ConfigId::new(self.mint()))
            }
            DesktopCommand::CaptureOutput(_) => Ticket::Capture(CaptureId::new(self.mint())),
            _ => Ticket::Sent,
        };
        self.sent.push(command);
        Ok(ticket)
    }

    fn drain_events(&mut self) -> Vec<DesktopEvent> {
        std::mem::take(&mut self.applied)
    }
}
