//! Whether a Quire command's targets exist.

use crate::backend::DesktopBackend;
use crate::command::DesktopCommand;
use crate::error::CommandError;
use crate::keys::ToplevelKey;
use crate::state::DesktopState;

/// `Ok` when every object the Quire-arm `command` names is in `state`; otherwise which is
/// not. The wire client resolves the same objects against its handles; the fake calls this.
///
/// Commands that predate the Quire arm are checked only when the offer chose the Quire arm and
/// the command needs a window's identifier (`SetMinimiseTarget`).
///
/// ```
/// use desktop_client::quire::check_targets;
/// use desktop_client::{AppKey, CommandError, DesktopCommand, DesktopState};
///
/// let state = DesktopState::default();
/// assert_eq!(
///     check_targets(&state, &DesktopCommand::QuitApp(AppKey::new(1))),
///     Err(CommandError::UnknownApp(AppKey::new(1)))
/// );
/// ```
pub fn check_targets(state: &DesktopState, command: &DesktopCommand) -> Result<(), CommandError> {
    match command {
        DesktopCommand::Place { toplevel, .. } | DesktopCommand::AssignSpaces { toplevel, .. } => {
            window(state, *toplevel)
        }
        DesktopCommand::MoveToSpace { toplevel, space } => {
            window(state, *toplevel)?;
            state
                .quire
                .space(*space)
                .map(|_| ())
                .ok_or(CommandError::UnknownSpace(*space))
        }
        DesktopCommand::ClearMinimiseTarget(toplevel) => window(state, *toplevel),
        DesktopCommand::SetMinimiseTarget { toplevel, .. }
            if state.backend() == Some(DesktopBackend::Quire) =>
        {
            identified(state, *toplevel)
        }
        DesktopCommand::ActivateSpace(space)
        | DesktopCommand::RemoveSpace(space)
        | DesktopCommand::MoveSpace { space, .. } => state
            .quire
            .space(*space)
            .map(|_| ())
            .ok_or(CommandError::UnknownSpace(*space)),
        DesktopCommand::ActivateApp(app)
        | DesktopCommand::HideApp(app)
        | DesktopCommand::UnhideApp(app)
        | DesktopCommand::HideOtherApps(app)
        | DesktopCommand::QuitApp(app)
        | DesktopCommand::ForceQuitApp(app)
        | DesktopCommand::ReopenApp(app) => state
            .quire
            .app(*app)
            .map(|_| ())
            .ok_or(CommandError::UnknownApp(*app)),
        // No target of the Quire state to look for (the minimise target's identifier is only
        // checked on the Quire arm, above).
        DesktopCommand::Activate(_)
        | DesktopCommand::Close(_)
        | DesktopCommand::Minimise(_)
        | DesktopCommand::Unminimise(_)
        | DesktopCommand::Maximise(_)
        | DesktopCommand::Unmaximise(_)
        | DesktopCommand::Fullscreen { .. }
        | DesktopCommand::Unfullscreen(_)
        | DesktopCommand::SetMinimiseTarget { .. }
        | DesktopCommand::ActivateWorkspace(_)
        | DesktopCommand::DeactivateWorkspace(_)
        | DesktopCommand::AssignWorkspace { .. }
        | DesktopCommand::RemoveWorkspace(_)
        | DesktopCommand::CreateWorkspace { .. }
        | DesktopCommand::CommitWorkspaces
        | DesktopCommand::ApplyOutputs(_)
        | DesktopCommand::TestOutputs(_)
        | DesktopCommand::CaptureOutput(_)
        | DesktopCommand::CreateSpace { .. }
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
        | DesktopCommand::SetNewSpaceTarget { .. }
        | DesktopCommand::AnswerPermission { .. }
        | DesktopCommand::AnswerTrial { .. }
        | DesktopCommand::SetFullScreenBar(_)
        | DesktopCommand::SetMotion(_) => Ok(()),
    }
}

fn window(state: &DesktopState, key: ToplevelKey) -> Result<(), CommandError> {
    state
        .quire
        .window(key)
        .map(|_| ())
        .ok_or(CommandError::UnknownToplevel(key))
}

fn identified(state: &DesktopState, key: ToplevelKey) -> Result<(), CommandError> {
    let toplevel = state
        .toplevels
        .get(key)
        .ok_or(CommandError::UnknownToplevel(key))?;
    toplevel
        .identifier
        .as_ref()
        .map(|_| ())
        .ok_or(CommandError::NoIdentifier(key))
}
