//! What a consumer can ask the compositor to do.

use crate::capture::CaptureRequest;
use crate::geometry::Rect;
use crate::keys::{CaptureId, ConfigId, GroupKey, SurfaceKey, ToplevelKey, WorkspaceKey};
use crate::names::OutputName;
use crate::output::OutputConfig;

/// A request to the compositor. Targets are keys from [`crate::DesktopState`].
///
/// The window-placement commands a Quire compositor offers are not here: wlr cannot place a
/// window (FINDINGS). They arrive with the Quire arm at P2.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum DesktopCommand {
    /// Activate on the seat the client picked at bind time.
    Activate(ToplevelKey),
    /// Ask the toplevel to close.
    Close(ToplevelKey),
    /// Minimise.
    Minimise(ToplevelKey),
    /// Restore from minimised.
    Unminimise(ToplevelKey),
    /// Maximise.
    Maximise(ToplevelKey),
    /// Leave maximised.
    Unmaximise(ToplevelKey),
    /// Go full screen, on `output` or on the compositor's choice.
    Fullscreen {
        /// The toplevel.
        toplevel: ToplevelKey,
        /// Where; `None` lets the compositor choose.
        output: Option<OutputName>,
    },
    /// Leave full screen.
    Unfullscreen(ToplevelKey),
    /// Tell the compositor where the minimised window's tile is: `rect` is in the coordinates
    /// of `surface`, a surface the caller registered.
    SetMinimiseTarget {
        /// The toplevel.
        toplevel: ToplevelKey,
        /// The caller's surface the rectangle is relative to.
        surface: SurfaceKey,
        /// The tile.
        rect: Rect,
    },
    /// Stage activating a workspace; takes effect at [`DesktopCommand::CommitWorkspaces`].
    ActivateWorkspace(WorkspaceKey),
    /// Stage deactivating a workspace.
    DeactivateWorkspace(WorkspaceKey),
    /// Stage moving a workspace into a group.
    AssignWorkspace {
        /// The workspace.
        workspace: WorkspaceKey,
        /// The group.
        group: GroupKey,
    },
    /// Stage removing a workspace.
    RemoveWorkspace(WorkspaceKey),
    /// Stage creating a workspace in a group.
    CreateWorkspace {
        /// The group.
        group: GroupKey,
        /// Its name.
        name: String,
    },
    /// Send the staged workspace requests (`ext_workspace_manager_v1.commit`).
    CommitWorkspaces,
    /// Apply an output configuration.
    ApplyOutputs(OutputConfig),
    /// Test an output configuration without applying it.
    TestOutputs(OutputConfig),
    /// Capture one frame of an output.
    CaptureOutput(CaptureRequest),
}

/// The variant of a [`DesktopCommand`] without its data, for errors and logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CommandKind {
    /// [`DesktopCommand::Activate`].
    Activate,
    /// [`DesktopCommand::Close`].
    Close,
    /// [`DesktopCommand::Minimise`].
    Minimise,
    /// [`DesktopCommand::Unminimise`].
    Unminimise,
    /// [`DesktopCommand::Maximise`].
    Maximise,
    /// [`DesktopCommand::Unmaximise`].
    Unmaximise,
    /// [`DesktopCommand::Fullscreen`].
    Fullscreen,
    /// [`DesktopCommand::Unfullscreen`].
    Unfullscreen,
    /// [`DesktopCommand::SetMinimiseTarget`].
    SetMinimiseTarget,
    /// [`DesktopCommand::ActivateWorkspace`].
    ActivateWorkspace,
    /// [`DesktopCommand::DeactivateWorkspace`].
    DeactivateWorkspace,
    /// [`DesktopCommand::AssignWorkspace`].
    AssignWorkspace,
    /// [`DesktopCommand::RemoveWorkspace`].
    RemoveWorkspace,
    /// [`DesktopCommand::CreateWorkspace`].
    CreateWorkspace,
    /// [`DesktopCommand::CommitWorkspaces`].
    CommitWorkspaces,
    /// [`DesktopCommand::ApplyOutputs`].
    ApplyOutputs,
    /// [`DesktopCommand::TestOutputs`].
    TestOutputs,
    /// [`DesktopCommand::CaptureOutput`].
    CaptureOutput,
}

impl DesktopCommand {
    /// Which command this is.
    pub fn kind(&self) -> CommandKind {
        match self {
            Self::Activate(_) => CommandKind::Activate,
            Self::Close(_) => CommandKind::Close,
            Self::Minimise(_) => CommandKind::Minimise,
            Self::Unminimise(_) => CommandKind::Unminimise,
            Self::Maximise(_) => CommandKind::Maximise,
            Self::Unmaximise(_) => CommandKind::Unmaximise,
            Self::Fullscreen { .. } => CommandKind::Fullscreen,
            Self::Unfullscreen(_) => CommandKind::Unfullscreen,
            Self::SetMinimiseTarget { .. } => CommandKind::SetMinimiseTarget,
            Self::ActivateWorkspace(_) => CommandKind::ActivateWorkspace,
            Self::DeactivateWorkspace(_) => CommandKind::DeactivateWorkspace,
            Self::AssignWorkspace { .. } => CommandKind::AssignWorkspace,
            Self::RemoveWorkspace(_) => CommandKind::RemoveWorkspace,
            Self::CreateWorkspace { .. } => CommandKind::CreateWorkspace,
            Self::CommitWorkspaces => CommandKind::CommitWorkspaces,
            Self::ApplyOutputs(_) => CommandKind::ApplyOutputs,
            Self::TestOutputs(_) => CommandKind::TestOutputs,
            Self::CaptureOutput(_) => CommandKind::CaptureOutput,
        }
    }
}

/// What sending a command hands back: nothing, or the number to wait on.
///
/// A configuration or a capture finishes later; its result arrives as an event carrying the
/// same id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Ticket {
    /// Sent; no result will follow.
    Sent,
    /// An output configuration is pending; see `OutputEvent::ConfigResolved`.
    Config(ConfigId),
    /// A capture is pending; see `DesktopEvent::Capture` and `Captures::take_ready`.
    Capture(CaptureId),
}
