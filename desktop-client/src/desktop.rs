//! The seam a consumer codes against: the wire client or the fake.

use crate::command::{DesktopCommand, Ticket};
use crate::event::DesktopEvent;
use crate::state::DesktopState;

/// A desktop connection: state in, commands out.
///
/// `client::DesktopClient` implements it over a Wayland connection and `fake::FakeDesktop`
/// over a script, so sill and cua test against the second and run on the first.
pub trait Desktop {
    /// Why a command fails to send.
    type Error: std::error::Error + 'static;

    /// Everything the compositor has said so far, reduced.
    fn state(&self) -> &DesktopState;

    /// Send a command.
    fn send(&mut self, command: DesktopCommand) -> Result<Ticket, Self::Error>;

    /// The events reduced into the state since the last call, oldest first.
    fn drain_events(&mut self) -> Vec<DesktopEvent>;
}
