//! What the Quire protocols report.

use super::app::AppEvent;
use super::shell::ShellEvent;
use super::space::SpaceEvent;
use super::window::WindowEvent;
use crate::keys::{AppKey, ToplevelKey};
use casement_protocols::vocab::ActorKind;

/// An event of the Quire arm.
///
/// ```
/// use desktop_client::quire::{AppChange, AppEvent, QuireEvent};
/// use desktop_client::{AppKey, DesktopEvent, DesktopState, reduce};
/// use casement_protocols::vocab::AppLifeKind;
///
/// let app = AppKey::new(7);
/// let events = [
///     AppEvent::Opened(app),
///     AppEvent::Changed { key: app, change: AppChange::Life(AppLifeKind::Running) },
///     AppEvent::Done(app),
/// ];
/// let state = events
///     .into_iter()
///     .fold(DesktopState::default(), |s, e| reduce(s, DesktopEvent::Quire(QuireEvent::App(e))));
/// assert_eq!(state.quire.apps().next().unwrap().life, AppLifeKind::Running);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum QuireEvent {
    /// An app event.
    App(AppEvent),
    /// A window event (the window is keyed by its ext handle's key).
    Window(WindowEvent),
    /// A Space event.
    Space(SpaceEvent),
    /// The active app and key window changed (`quire_desktop_manager_v1.active`).
    Active {
        /// The active app; `None` when no app is active.
        app: Option<AppKey>,
        /// The key window; `None` when the active app has none.
        key: Option<ToplevelKey>,
        /// Who caused it.
        actor: ActorKind,
    },
    /// A note from `quire_shell_v1`.
    Shell(ShellEvent),
}
