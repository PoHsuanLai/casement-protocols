//! Sending a command.

use super::error::WireError;
use super::wire::Wire;
use crate::command::{DesktopCommand, Ticket};

/// Checks `command` with `supports`, resolves its keys against the handles and sends it.
/// A key with no handle is `CommandError::Unknown…`; workspace requests are staged until
/// `CommitWorkspaces`; a configuration is validated against the state first and completed
/// with every unmentioned enabled head.
pub(super) fn send(wire: &mut Wire, command: DesktopCommand) -> Result<Ticket, WireError> {
    let _ = (wire, command);
    todo!("fill: match on the command; one function per protocol")
}
