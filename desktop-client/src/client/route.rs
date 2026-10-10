//! Where the connection comes from.

use std::os::fd::OwnedFd;
use std::os::unix::net::UnixStream;

use wayland_client::Connection;

use super::error::WireError;

/// How the caller reaches the compositor.
///
/// The client reads the environment only for `Environment`, which the caller chooses. A shell
/// asks the compositor for its own connection (`org.quire.Compositor1.Connect("shell")`) and
/// passes the descriptor as `Handed`; the compositor then knows the role without trusting a
/// name. The same shape as shell-host's `WaylandRoute`.
///
/// ```no_run
/// use desktop_client::client::{DesktopClient, WaylandRoute};
/// use desktop_client::DesktopConfig;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = DesktopClient::connect(WaylandRoute::Environment, DesktopConfig::default())?;
/// # let _ = client;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub enum WaylandRoute {
    /// Connect the way any Wayland client does: `WAYLAND_SOCKET`, then `WAYLAND_DISPLAY`.
    Environment,
    /// Use this connected socket (the compositor's end already holds the role).
    Handed(OwnedFd),
}

impl WaylandRoute {
    /// Opens the connection this route names.
    pub fn open(self) -> Result<Connection, WireError> {
        Ok(match self {
            Self::Environment => Connection::connect_to_env()?,
            Self::Handed(fd) => Connection::from_socket(UnixStream::from(fd))?,
        })
    }
}
