//! Whether this client may send a command: offered, and bound.

use crate::backend::backend_for;
use crate::command::DesktopCommand;
use crate::config::{DesktopConfig, ShellBinding};
use crate::error::CommandError;
use crate::protocol::{Protocol, ProtocolSet};
use crate::supports::{missing, requires_on};

/// `Ok` when `command` can go on the wire of a client configured with `config` on a compositor
/// that offers `offered`; otherwise why not.
///
/// A missing protocol is [`CommandError::Unsupported`]. `quire_shell_v1` offered but not bound
/// (the caller did not say it holds the shell role) is [`CommandError::NotBound`]. The wire
/// client and the fake both call it, so they agree.
///
/// ```
/// use desktop_client::{
///     CommandError, DesktopCommand, DesktopConfig, Protocol, ProtocolSet, admit,
/// };
///
/// let offered = ProtocolSet::empty().with(Protocol::QuireShell);
/// let cancel = DesktopCommand::CancelSwitcher;
/// assert!(matches!(
///     admit(&offered, &DesktopConfig::default(), &cancel),
///     Err(CommandError::NotBound { .. })
/// ));
/// assert!(admit(&offered, &DesktopConfig::holding_shell(), &cancel).is_ok());
/// ```
pub fn admit(
    offered: &ProtocolSet,
    config: &DesktopConfig,
    command: &DesktopCommand,
) -> Result<(), CommandError> {
    if let Some(needs) = missing(offered, command) {
        return Err(CommandError::Unsupported {
            command: command.kind(),
            needs,
        });
    }
    let needs_shell = requires_on(backend_for(offered), command).contains(&Protocol::QuireShell);
    if needs_shell && config.shell == ShellBinding::Skip {
        return Err(CommandError::NotBound {
            command: command.kind(),
            needs: Protocol::QuireShell,
        });
    }
    Ok(())
}
