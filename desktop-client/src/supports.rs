//! Whether the offered protocols can carry a command.

use crate::backend::{DesktopBackend, backend_for};
use crate::command::DesktopCommand;
use crate::protocol::{Protocol, ProtocolSet};

/// The answer of [`supports`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Supports {
    /// Every protocol the command needs is offered.
    Yes,
    /// At least one is missing.
    No,
}

/// What a window command needs on the Quire arm: its model and the ext list that names windows.
const QUIRE_WINDOW: &[Protocol] = &[Protocol::QuireDesktop, Protocol::ExtForeignToplevelList];
/// What an app or Space command needs.
const QUIRE_DESKTOP: &[Protocol] = &[Protocol::QuireDesktop];
/// What a shell command needs: bound only by a client that holds the shell role.
const QUIRE_SHELL: &[Protocol] = &[Protocol::QuireShell];
/// What the window commands need without the Quire arm.
const WLR_TOPLEVEL: &[Protocol] = &[Protocol::WlrForeignToplevelManagement];

/// The protocols `command` needs on the wlr arm, all of them. For a Quire-only command, the
/// Quire protocols it needs. See [`requires_on`] for the arm an offer chooses.
pub fn requires(command: &DesktopCommand) -> &'static [Protocol] {
    requires_on(Some(DesktopBackend::Wlr), command)
}

/// The protocols `command` needs when the offer chose `backend` (see
/// [`backend_for`]), all of them.
///
/// The plain window commands (`Activate`, `Close`, `Minimise`, ...) go to
/// `quire_desktop_window_v1` on the Quire arm and to `zwlr-foreign-toplevel-management` on
/// every other; the minimise target goes to `quire_shell_v1` on the Quire arm.
///
/// ```
/// use desktop_client::{DesktopBackend, DesktopCommand, Protocol, ToplevelKey, requires_on};
///
/// let close = DesktopCommand::Close(ToplevelKey::new(1));
/// assert_eq!(
///     requires_on(Some(DesktopBackend::Wlr), &close),
///     &[Protocol::WlrForeignToplevelManagement]
/// );
/// assert!(requires_on(Some(DesktopBackend::Quire), &close).contains(&Protocol::QuireDesktop));
/// ```
pub fn requires_on(
    backend: Option<DesktopBackend>,
    command: &DesktopCommand,
) -> &'static [Protocol] {
    use Protocol::*;
    let quire = backend == Some(DesktopBackend::Quire);
    let by_arm = |on_quire: &'static [Protocol]| if quire { on_quire } else { WLR_TOPLEVEL };
    match command {
        DesktopCommand::Activate(_)
        | DesktopCommand::Close(_)
        | DesktopCommand::Minimise(_)
        | DesktopCommand::Unminimise(_)
        | DesktopCommand::Maximise(_)
        | DesktopCommand::Unmaximise(_)
        | DesktopCommand::Fullscreen { .. }
        | DesktopCommand::Unfullscreen(_) => by_arm(QUIRE_WINDOW),
        DesktopCommand::SetMinimiseTarget { .. } => by_arm(QUIRE_SHELL),
        DesktopCommand::ActivateWorkspace(_)
        | DesktopCommand::DeactivateWorkspace(_)
        | DesktopCommand::AssignWorkspace { .. }
        | DesktopCommand::RemoveWorkspace(_)
        | DesktopCommand::CreateWorkspace { .. }
        | DesktopCommand::CommitWorkspaces => &[ExtWorkspace],
        DesktopCommand::ApplyOutputs(_) | DesktopCommand::TestOutputs(_) => &[WlrOutputManagement],
        DesktopCommand::CaptureOutput(request) if request.space.is_some() => {
            &[ExtImageCopyCapture, QuireShell]
        }
        DesktopCommand::CaptureOutput(_) => &[ExtImageCopyCapture, ExtOutputImageCaptureSource],
        DesktopCommand::Place { .. }
        | DesktopCommand::MoveToSpace { .. }
        | DesktopCommand::AssignSpaces { .. } => QUIRE_WINDOW,
        DesktopCommand::ActivateSpace(_)
        | DesktopCommand::RemoveSpace(_)
        | DesktopCommand::MoveSpace { .. }
        | DesktopCommand::CreateSpace { .. }
        | DesktopCommand::ActivateApp(_)
        | DesktopCommand::HideApp(_)
        | DesktopCommand::UnhideApp(_)
        | DesktopCommand::HideOtherApps(_)
        | DesktopCommand::QuitApp(_)
        | DesktopCommand::ForceQuitApp(_)
        | DesktopCommand::ReopenApp(_) => QUIRE_DESKTOP,
        DesktopCommand::ClearMinimiseTarget(_)
        | DesktopCommand::SetShellRole { .. }
        | DesktopCommand::PickSwitcher(_)
        | DesktopCommand::CancelSwitcher
        | DesktopCommand::Overview { .. }
        | DesktopCommand::ShowDesktop(_)
        | DesktopCommand::PickWindowMenu { .. }
        | DesktopCommand::PickSplit(_)
        | DesktopCommand::CancelSplit
        | DesktopCommand::DivideSplit { .. }
        | DesktopCommand::SetSpaceTargets { .. }
        | DesktopCommand::SetNewSpaceTarget { .. } => QUIRE_SHELL,
    }
}

/// The first protocol `command` needs that `offered` lacks, on the arm `offered` chooses.
pub fn missing(offered: &ProtocolSet, command: &DesktopCommand) -> Option<Protocol> {
    requires_on(backend_for(offered), command)
        .iter()
        .copied()
        .find(|p| !offered.contains(*p))
}

/// Whether `offered` carries `command`. The compositor may still refuse at run time (a
/// workspace that lacks the ability, a closed toplevel); this answers the protocol question.
/// Whether this client bound `quire_shell_v1` is the caller's choice, not the offer's; see
/// `admit`.
pub fn supports(offered: &ProtocolSet, command: &DesktopCommand) -> Supports {
    match missing(offered, command) {
        None => Supports::Yes,
        Some(_) => Supports::No,
    }
}
