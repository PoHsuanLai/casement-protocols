//! Wire-layer errors.

use wayland_client::backend::WaylandError;
use wayland_client::globals::{BindError, GlobalError};
use wayland_client::{ConnectError, DispatchError};

use crate::error::CommandError;

/// A failure of the wire layer.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum WireError {
    /// The registry could not be read.
    #[error("reading the registry failed")]
    Registry(#[from] GlobalError),
    /// A global that was offered could not be bound.
    #[error("binding a global failed")]
    Bind(#[from] BindError),
    /// Reading or dispatching events failed (the connection is likely gone).
    #[error("dispatching events failed")]
    Dispatch(#[from] DispatchError),
    /// Writing the requests to the socket failed.
    #[error("sending requests failed")]
    Flush(#[from] WaylandError),
    /// Connecting failed (only for callers that use the helpers on `Connection`).
    #[error("connecting failed")]
    Connect(#[from] ConnectError),
    /// The shared-memory buffer for a capture could not be made.
    #[error("creating the capture buffer failed")]
    Shm(#[from] std::io::Error),
    /// The command was refused before it went on the wire.
    #[error("the command was refused")]
    Command(#[from] CommandError),
    /// The compositor's `wl_output` is older than version 4, which carries the output name every
    /// toplevel, workspace and head is matched on.
    #[error("wl_output global {global} is version {version}; version 4 (name) is needed")]
    OutputTooOld {
        /// The registry name of the global.
        global: u32,
        /// The version it offers.
        version: u32,
    },
}
