//! What the caller says about itself when it connects.

/// Whether this client holds the shell role and so binds `quire_shell_v1`.
///
/// The compositor advertises `quire_shell_v1` on Shell connections only; the caller knows which
/// connection it holds, so it says so here and the library never guesses from names or the
/// environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ShellBinding {
    /// Do not bind `quire_shell_v1`, even if it is offered.
    #[default]
    Skip,
    /// Bind `quire_shell_v1` when it is offered.
    Bind,
}

/// The caller's choices for one desktop connection.
///
/// ```
/// use desktop_client::{DesktopConfig, ShellBinding};
///
/// assert_eq!(DesktopConfig::default().shell, ShellBinding::Skip);
/// assert_eq!(DesktopConfig::holding_shell().shell, ShellBinding::Bind);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct DesktopConfig {
    /// Whether to bind `quire_shell_v1`.
    pub shell: ShellBinding,
}

impl DesktopConfig {
    /// A client on a Shell connection: it binds `quire_shell_v1` as well.
    pub const fn holding_shell() -> Self {
        Self {
            shell: ShellBinding::Bind,
        }
    }
}
