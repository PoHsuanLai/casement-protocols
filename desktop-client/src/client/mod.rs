//! The wire layer: binds the globals on a connection the caller supplies.
//!
//! [`DesktopClient`] owns one event queue on that connection. It reads no environment variable
//! and opens no socket; the caller connects (from `WAYLAND_DISPLAY`, a handed-over fd, a test
//! compositor) and passes the [`Connection`].
//!
//! Protocol events become [`DesktopEvent`]s, are reduced into the state and are kept for
//! [`Desktop::drain_events`]. Commands are checked with `supports` first and then sent.

// Freeze: the fill lane calls the stubs below; until then they are unused.
#![allow(dead_code)]

mod bind;
mod capture;
mod commands;
mod error;
mod handles;
mod output;
mod shm;
mod toplevel;
mod wire;
mod workspace;

pub use error::WireError;

use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::{Connection, EventQueue};

use crate::capture::{CaptureFailure, Frame};
use crate::command::{DesktopCommand, Ticket};
use crate::desktop::Desktop;
use crate::event::DesktopEvent;
use crate::keys::{CaptureId, SurfaceKey};
use crate::state::DesktopState;
use wire::Wire;

/// A desktop client on one Wayland connection.
///
/// ```no_run
/// use desktop_client::Desktop;
/// use desktop_client::client::DesktopClient;
/// use wayland_client::Connection;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let connection = Connection::connect_to_env()?; // the caller's choice, not the library's
/// let mut client = DesktopClient::new(&connection)?;
/// client.roundtrip()?;
/// for toplevel in client.state().toplevels.announced() {
///     println!("{}", toplevel.title);
/// }
/// # Ok(())
/// # }
/// ```
pub struct DesktopClient {
    wire: Wire,
    queue: EventQueue<Wire>,
    connection: Connection,
}

impl DesktopClient {
    /// Binds every global of the protocols the compositor offers (each is optional), takes the
    /// first seat for activation, and runs round trips until the first descriptions are in.
    /// The state's [`crate::DesktopState::protocols`] says what was found.
    pub fn new(connection: &Connection) -> Result<Self, WireError> {
        let _ = connection;
        todo!("fill: registry_queue_init, bind per Protocol, two roundtrips, emit Globals")
    }

    /// The connection the client was given, for polling its file descriptor.
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    /// Processes the events already read, without blocking. Returns how many were dispatched.
    pub fn dispatch_pending(&mut self) -> Result<usize, WireError> {
        todo!("fill: queue.dispatch_pending(&mut wire)")
    }

    /// Reads from the socket and dispatches, blocking until something arrives.
    pub fn blocking_dispatch(&mut self) -> Result<usize, WireError> {
        todo!("fill: queue.blocking_dispatch(&mut wire)")
    }

    /// Waits until the compositor has answered everything sent so far.
    pub fn roundtrip(&mut self) -> Result<usize, WireError> {
        todo!("fill: queue.roundtrip(&mut wire)")
    }

    /// Registers a surface of the caller's so a command can name it
    /// (`DesktopCommand::SetMinimiseTarget`).
    pub fn register_surface(&mut self, surface: &WlSurface) -> SurfaceKey {
        let _ = surface;
        todo!("fill: store the surface under a fresh key")
    }

    /// Forgets a registered surface.
    pub fn forget_surface(&mut self, key: SurfaceKey) {
        let _ = key;
        todo!("fill: drop the surface")
    }

    /// Takes a finished capture: its frame, or why it failed. `None` while it is still going.
    pub fn take_capture(&mut self, id: CaptureId) -> Option<Result<Frame, CaptureFailure>> {
        let _ = id;
        todo!("fill: Captures::take_ready on the state")
    }
}

impl Desktop for DesktopClient {
    type Error = WireError;

    fn state(&self) -> &DesktopState {
        &self.wire.state
    }

    fn send(&mut self, command: DesktopCommand) -> Result<Ticket, WireError> {
        commands::send(&mut self.wire, command)
    }

    fn drain_events(&mut self) -> Vec<DesktopEvent> {
        std::mem::take(&mut self.wire.events)
    }
}

impl std::fmt::Debug for DesktopClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DesktopClient")
            .field("state", &self.wire.state)
            .finish_non_exhaustive()
    }
}
