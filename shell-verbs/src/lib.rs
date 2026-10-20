//! The verbs the compositor sends the desktop shell, as one type both sides share.
//!
//! The compositor's keymap binds a chord to a [`ShellVerb`]; the shell's command line parses the
//! same words back into it. A rename or a new verb is then a compile error on both sides, not a
//! string that silently stops matching. The crate has no dependencies.
//!
//! Mission Control, Spaces, full screen and window placement are not here: they are the
//! compositor's own commands, and the shell is never asked for them.

mod verb;

pub use verb::{ALL, ShellVerb};
