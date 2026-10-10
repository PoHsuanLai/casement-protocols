//! Why a command cannot be sent.

use crate::command::CommandKind;
use crate::keys::{GroupKey, SurfaceKey, ToplevelKey, WorkspaceKey};
use crate::names::OutputName;
use crate::output::ConfigError;
use crate::protocol::Protocol;

/// A command that was refused before anything went on the wire.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum CommandError {
    /// The compositor did not offer a protocol the command needs.
    #[error("{command:?} needs {needs}, which the compositor does not offer")]
    Unsupported {
        /// The command.
        command: CommandKind,
        /// The first missing protocol.
        needs: Protocol,
    },
    /// No such toplevel (never seen, or closed).
    #[error("no toplevel {0:?}")]
    UnknownToplevel(ToplevelKey),
    /// No such workspace.
    #[error("no workspace {0:?}")]
    UnknownWorkspace(WorkspaceKey),
    /// No such workspace group.
    #[error("no workspace group {0:?}")]
    UnknownGroup(GroupKey),
    /// The caller never registered this surface.
    #[error("no registered surface {0:?}")]
    UnknownSurface(SurfaceKey),
    /// No output of that name.
    #[error("no output {0:?}")]
    UnknownOutput(OutputName),
    /// The output configuration is not valid against the current heads.
    #[error("invalid output configuration")]
    Config(#[from] ConfigError),
    /// The seat the client acts on has gone, so nothing can be activated.
    #[error("no seat to act on")]
    NoSeat,
}
