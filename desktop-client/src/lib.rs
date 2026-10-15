//! One typed Wayland desktop client, portable to any wlroots-style compositor.
//!
//! It tracks five things the desktop shell and the agent tier both need:
//!
//! - toplevels: identity from `ext-foreign-toplevel-list-v1`, state and actions from
//!   `zwlr-foreign-toplevel-management-v1` ([`toplevel`]);
//! - workspaces: `ext-workspace-v1` ([`workspace`]);
//! - outputs: `zwlr-output-management-v1`, including applying and testing a configuration
//!   ([`output`]);
//! - output capture: `ext-image-copy-capture-v1` over `ext-output-image-capture-source-v1`,
//!   one frame of an output into shared memory ([`capture`]);
//! - the Quire arm: apps, windows (role, place, geometry, emphasis, placement), Spaces and the
//!   shell's notes (modifiers, switcher, overview, hot corners, reveals) from
//!   `quire_desktop_v1` and `quire_shell_v1` ([`quire`]). [`backend_for`] picks it from the
//!   offered globals when `quire_desktop_v1` and the ext list are both there; otherwise the
//!   wlr arm serves the toplevels, and the Quire commands are refused by [`supports`].
//!
//! # Layers
//!
//! - The core (always built, no Wayland, no I/O): the state types, the events, the pure
//!   transition [`reduce`] over [`DesktopState`], the [`DesktopCommand`]s and
//!   [`supports`], which answers from the offered [`ProtocolSet`].
//! - `client` (feature): `client::DesktopClient` binds the globals on a `wayland_client`
//!   `Connection` the caller supplies, turns protocol events into [`DesktopEvent`]s and sends
//!   commands. The library reads no environment variable and opens no socket on its own: the
//!   caller connects, or names the route (`client::WaylandRoute`: the environment, or a
//!   connection handed over as a file descriptor, as `Compositor1.Connect("shell")` returns).
//! - `fake` (feature): `fake::FakeDesktop` takes scripted events and records the commands, so
//!   a consumer tests against the same [`Desktop`] trait without a compositor.
//!
//! A consumer maps this crate's state into its own model (sill's `DesktopView`); application
//! grouping, most-recently-used order and Spaces are consumer concerns, not protocol facts.
//!
//! # Example
//!
//! Ask whether the compositor can do something before offering it:
//!
//! ```
//! use desktop_client::{DesktopCommand, ProtocolSet, Supports, ToplevelKey, supports};
//!
//! let offered = ProtocolSet::from_interfaces([
//!     "ext_foreign_toplevel_list_v1",
//!     "zwlr_foreign_toplevel_manager_v1",
//! ]);
//! let close = DesktopCommand::Close(ToplevelKey::new(1));
//! assert_eq!(supports(&offered, &close), Supports::Yes);
//! assert_eq!(supports(&ProtocolSet::empty(), &close), Supports::No);
//! ```

pub mod capture;
pub mod command;
pub mod desktop;
pub mod error;
pub mod geometry;
pub mod keys;
pub mod names;
pub mod output;
pub mod protocol;
pub mod quire;
pub mod toplevel;
pub mod workspace;

mod admit;
mod backend;
mod config;
mod event;
mod reduce;
mod state;
mod supports;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "fake")]
pub mod fake;

pub use admit::admit;
pub use backend::{DesktopBackend, backend_for};
pub use capture::{CaptureRequest, Frame};
pub use command::{CommandKind, DesktopCommand, SpaceTarget, Ticket};
pub use config::{DesktopConfig, ShellBinding};
pub use desktop::Desktop;
pub use error::CommandError;
pub use event::DesktopEvent;
pub use geometry::{Point, Rect, Size};
pub use keys::{
    AppKey, CaptureId, ConfigId, GroupKey, HeadKey, ModeKey, SpaceKey, SurfaceKey, ToplevelKey,
    WorkspaceKey,
};
pub use names::{AppId, OutputName, SpaceId, ToplevelIdentifier};
pub use protocol::{Protocol, ProtocolSet};
pub use reduce::reduce;
pub use state::DesktopState;
pub use supports::{Supports, missing, requires, requires_on, supports};
