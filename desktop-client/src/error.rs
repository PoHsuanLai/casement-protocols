//! Why a command cannot be sent.

use crate::command::CommandRoute;
use crate::keys::{AppKey, GroupKey, SpaceKey, SurfaceKey, ToplevelKey, WorkspaceKey};
use crate::names::OutputName;
use crate::output::ConfigError;
use crate::protocol::Protocol;

/// A command that was refused before anything went on the wire.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum CommandError {
    /// The compositor did not offer a protocol the command needs.
    #[error("a {route:?} command needs {needs}, which the compositor does not offer")]
    Unsupported {
        /// The family the command belongs to.
        route: CommandRoute,
        /// The first missing protocol.
        needs: Protocol,
    },
    /// The compositor offers a protocol the command needs, but this client was not asked to bind
    /// it (see [`crate::DesktopConfig`]).
    #[error("a {route:?} command needs {needs}, which this client did not bind")]
    NotBound {
        /// The family the command belongs to.
        route: CommandRoute,
        /// The protocol that is offered and not bound.
        needs: Protocol,
    },
    /// No such toplevel (never seen, or closed).
    #[error("no toplevel {0:?}")]
    UnknownToplevel(ToplevelKey),
    /// No such app.
    #[error("no app {0:?}")]
    UnknownApp(AppKey),
    /// No such Space.
    #[error("no Space {0:?}")]
    UnknownSpace(SpaceKey),
    /// The toplevel has no identifier yet, which the shell protocol names windows by.
    #[error("toplevel {0:?} has no identifier")]
    NoIdentifier(ToplevelKey),
    /// A wire code the bindings do not know (the vocab and the bindings are tested to agree, so
    /// this is a version skew).
    #[error("no wire value has code {0}")]
    Code(u32),
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
