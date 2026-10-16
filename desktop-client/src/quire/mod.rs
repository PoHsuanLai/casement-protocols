//! The Quire arm: apps, windows and Spaces from `quire_desktop_v1`, and the shell's notes from
//! `quire_shell_v1`.
//!
//! A compositor that offers the private protocols (and the ext toplevel list, which names its
//! windows) reports what wlr cannot: an app's identity and life, a window's role, place,
//! geometry, emphasis and placement, Spaces with their order, and the shell's modifiers,
//! switcher and overview. [`QuireEvent`] carries all of it into the same pure
//! [`crate::reduce`]; [`Quire`] is the result, inside [`crate::DesktopState::quire`].
//!
//! A window is keyed by the [`crate::ToplevelKey`] of its ext handle, so title and app id stay
//! on [`crate::toplevel::Toplevel`], and the reducer projects the Quire state onto the
//! toplevel's flags (minimised, maximised, full screen, activated), parent and output so a
//! consumer that only reads toplevels keeps working. The wire layer links a window object to its
//! ext handle by identifier with [`Linker`], which is pure and tested.

mod app;
mod ask;
mod event;
mod link;
mod project;
mod reduce;
mod set;
mod shell;
mod space;
mod targets;
mod trial;
mod window;

pub use app::{AppChange, AppEvent, MenuSource, QuireApp};
pub use ask::PermissionAsk;
pub use event::QuireEvent;
pub use link::{Linker, WindowObject};
pub use set::{Active, Quire};
pub use shell::{
    Overview, OverviewSlot, OverviewSpaceSlot, Reveals, Shell, ShellEvent, SplitChoice, Switcher,
};
pub use space::{QuireSpace, SpaceChange, SpaceEvent};
pub use targets::check_targets;
pub use trial::DisplayTrial;
pub use window::{Lease, QuireWindow, SpacePlace, WindowChange, WindowEvent, WindowGeometry};

pub(crate) use project::project_toplevels;
pub(crate) use reduce::reduce_quire;
