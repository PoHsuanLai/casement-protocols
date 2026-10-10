//! A scripted desktop for tests: events in, commands out, no compositor.
//!
//! ```
//! use desktop_client::fake::FakeDesktop;
//! use desktop_client::{Desktop, DesktopCommand, ProtocolSet, ToplevelKey};
//!
//! let mut desktop = FakeDesktop::new(ProtocolSet::empty());
//! // No toplevel management offered: the command is refused and not recorded.
//! assert!(desktop.send(DesktopCommand::Close(ToplevelKey::new(1))).is_err());
//! assert!(desktop.sent().is_empty());
//! ```

mod script;

pub use script::{Advance, FakeDesktop};
