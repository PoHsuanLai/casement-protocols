//! The wire layer: binds the globals on a connection the caller supplies.
//!
//! [`DesktopClient`] owns one event queue on that connection. It reads no environment variable
//! and opens no socket by itself: the caller connects (a test compositor, say) and passes the
//! [`Connection`], or names a [`WaylandRoute`] (the environment, or a descriptor handed over by
//! `Compositor1.Connect("shell")`) and lets [`DesktopClient::connect`] open it.
//!
//! Protocol events become [`DesktopEvent`]s, are reduced into the state and are kept for
//! [`Desktop::drain_events`]. Commands are checked with `supports` first and then sent.

mod bind;
mod capture;
mod commands;
mod error;
mod handles;
mod output;
mod pairing;
mod passive;
mod quire;
mod quire_commands;
mod route;
mod shell;
mod shm;
mod toplevel;
mod wire;
mod workspace;

pub use error::WireError;
pub use route::WaylandRoute;

use std::task::{Context, Poll};

use wayland_client::globals::registry_queue_init;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::{Connection, EventQueue};

use crate::capture::{CaptureFailure, Frame};
use crate::command::{DesktopCommand, Ticket};
use crate::config::DesktopConfig;
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
        Self::with_config(connection, DesktopConfig::default())
    }

    /// Like [`DesktopClient::new`], for a caller that says more about itself: with
    /// `DesktopConfig::holding_shell()` it also binds `quire_shell_v1` when that is offered.
    ///
    /// On the Quire arm (`quire_desktop_v1` and the ext list offered) the toplevels come from
    /// those and the wlr manager is left unbound.
    pub fn with_config(connection: &Connection, config: DesktopConfig) -> Result<Self, WireError> {
        let (globals, mut queue) = registry_queue_init::<Wire>(connection)?;
        let qh = queue.handle();
        let mut wire = Wire::new(qh.clone(), config);
        wire.emit(DesktopEvent::Globals(bind::offered(&globals)));
        bind::bind_all(&mut wire, &globals, &qh)?;
        // The first round trip delivers the objects, the second their descriptions.
        queue.roundtrip(&mut wire)?;
        queue.roundtrip(&mut wire)?;
        Ok(Self {
            wire,
            queue,
            connection: connection.clone(),
        })
    }

    /// Opens the connection `route` names (the environment, or a descriptor handed over by
    /// `Compositor1.Connect("shell")`) and binds on it as [`DesktopClient::with_config`] does.
    pub fn connect(route: WaylandRoute, config: DesktopConfig) -> Result<Self, WireError> {
        Self::with_config(&route.open()?, config)
    }

    /// The connection the client was given, for polling its file descriptor.
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    /// Processes the events already read, without blocking. Returns how many were dispatched.
    pub fn dispatch_pending(&mut self) -> Result<usize, WireError> {
        Ok(self.queue.dispatch_pending(&mut self.wire)?)
    }

    /// Processes everything queued and leaves `cx`'s waker in the queue, so it is woken when the
    /// next event is queued. Returns how many [`DesktopEvent`]s the call added to
    /// [`Desktop::drain_events`].
    ///
    /// Another thread that shares the connection can read the socket first. Its read moves this
    /// client's events into the client's queue at once, and a caller that waits only on the
    /// socket's descriptor and libwayland's queue never learns of them. Calling this when the
    /// socket is readable, and again whenever the waker fires, dispatches those events on the
    /// caller's thread. The waker must be one the caller's loop can be woken by from any thread.
    pub fn poll_dispatch_pending(&mut self, cx: &mut Context<'_>) -> Result<usize, WireError> {
        let before = self.wire.events.len();
        match self.queue.poll_dispatch_pending(cx, &mut self.wire) {
            // The queue never ends: it is empty, and the waker is left in it.
            Poll::Pending => Ok(self.wire.events.len() - before),
            Poll::Ready(Err(error)) => Err(error.into()),
            Poll::Ready(Ok(never)) => match never {},
        }
    }

    /// Reads from the socket and dispatches, blocking until something arrives.
    pub fn blocking_dispatch(&mut self) -> Result<usize, WireError> {
        Ok(self.queue.blocking_dispatch(&mut self.wire)?)
    }

    /// Waits until the compositor has answered everything sent so far.
    pub fn roundtrip(&mut self) -> Result<usize, WireError> {
        Ok(self.queue.roundtrip(&mut self.wire)?)
    }

    /// Registers a surface of the caller's so a command can name it
    /// (`DesktopCommand::SetMinimiseTarget`, `DesktopCommand::SetShellRole`). The surface must
    /// belong to this client's connection; a surface of another connection cannot be named
    /// (the shell protocol names windows by identifier string for that reason).
    pub fn register_surface(&mut self, surface: &WlSurface) -> SurfaceKey {
        let key = SurfaceKey::new(self.wire.mint());
        self.wire.handles.surfaces.insert(key, surface.clone());
        key
    }

    /// Forgets a registered surface.
    pub fn forget_surface(&mut self, key: SurfaceKey) {
        self.wire.handles.surfaces.remove(&key);
    }

    /// Takes a finished capture: its frame, or why it failed. `None` while it is still going.
    pub fn take_capture(&mut self, id: CaptureId) -> Option<Result<Frame, CaptureFailure>> {
        let captures = std::mem::take(&mut self.wire.state.captures);
        let (captures, ready) = captures.take_ready(id);
        self.wire.state.captures = captures;
        ready
    }
}

impl Desktop for DesktopClient {
    type Error = WireError;

    fn state(&self) -> &DesktopState {
        &self.wire.state
    }

    fn send(&mut self, command: DesktopCommand) -> Result<Ticket, WireError> {
        let ticket = commands::send(&mut self.wire, command)?;
        self.connection.flush()?;
        Ok(ticket)
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
