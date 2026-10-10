//! The fake itself.

use std::collections::VecDeque;

use crate::admit::admit;
use crate::command::{DesktopCommand, Ticket};
use crate::config::DesktopConfig;
use crate::desktop::Desktop;
use crate::error::CommandError;
use crate::event::DesktopEvent;
use crate::keys::{CaptureId, ConfigId};
use crate::protocol::ProtocolSet;
use crate::quire::check_targets;
use crate::reduce::reduce;
use crate::state::DesktopState;

/// A [`Desktop`] fed by a script.
///
/// Queue events with [`FakeDesktop::script`]; [`FakeDesktop::advance`] reduces the next one
/// into the state (as the wire client does when a compositor speaks). Commands are checked
/// with `admit` like the real client does (offered, and bound as the
/// [`DesktopConfig`] says), the Quire commands' targets are looked up in the state, and then they
/// are recorded.
#[derive(Debug)]
pub struct FakeDesktop {
    state: DesktopState,
    config: DesktopConfig,
    script: VecDeque<DesktopEvent>,
    applied: Vec<DesktopEvent>,
    sent: Vec<DesktopCommand>,
    next_id: u64,
}

impl FakeDesktop {
    /// A fake whose compositor offers `protocols`.
    pub fn new(protocols: ProtocolSet) -> Self {
        Self::with_config(protocols, DesktopConfig::default())
    }

    /// A fake whose compositor offers `protocols`, for a client configured with `config`
    /// (`DesktopConfig::holding_shell()` binds `quire_shell_v1`).
    ///
    /// ```
    /// use desktop_client::fake::FakeDesktop;
    /// use desktop_client::{Desktop, DesktopCommand, DesktopConfig, Protocol, ProtocolSet};
    /// use casement_protocols::vocab::{OverviewMode, Toggle};
    ///
    /// let offered = ProtocolSet::empty().with(Protocol::QuireShell);
    /// let command = DesktopCommand::Overview {
    ///     mode: OverviewMode::MissionControl,
    ///     toggle: Toggle::On,
    /// };
    /// let mut skipped = FakeDesktop::new(offered.clone());
    /// assert!(skipped.send(command.clone()).is_err());
    /// let mut shell = FakeDesktop::with_config(offered, DesktopConfig::holding_shell());
    /// assert!(shell.send(command).is_ok());
    /// ```
    pub fn with_config(protocols: ProtocolSet, config: DesktopConfig) -> Self {
        Self {
            state: DesktopState::new(protocols),
            config,
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
        admit(&self.state.protocols, &self.config, &command)?;
        check_targets(&self.state, &command)?;
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
