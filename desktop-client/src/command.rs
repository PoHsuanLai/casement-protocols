//! What a consumer can ask the compositor to do.

use crate::capture::CaptureRequest;
use crate::geometry::Rect;
use crate::keys::{
    AppKey, CaptureId, ConfigId, GroupKey, SpaceKey, SurfaceKey, ToplevelKey, WorkspaceKey,
};
use crate::names::{OutputName, SpaceId, ToplevelIdentifier};
use crate::output::OutputConfig;
use casement_protocols::vocab::{
    Answer, AskId, Assign, OverviewMode, PlaceTarget, ShellRole, Toggle, TrialAnswer, TrialId,
    WindowEntry,
};

/// A request to the compositor. Targets are keys from [`crate::DesktopState`].
///
/// The commands from `Place` on are the Quire arm's: wlr cannot place a window, move it to a
/// Space or tell an app to hide (FINDINGS). Which protocol carries each command is
/// [`crate::requires_on`]'s table; [`crate::supports`] answers it for an offer.
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
    /// Place a window: a tile zone, fill, centre or restore. Quire arm.
    Place {
        /// The toplevel.
        toplevel: ToplevelKey,
        /// Where.
        target: PlaceTarget,
    },
    /// Move a window to a Space. Quire arm.
    MoveToSpace {
        /// The toplevel.
        toplevel: ToplevelKey,
        /// The Space.
        space: SpaceKey,
    },
    /// Assign a window to every Space, the one it is on, or none. Quire arm.
    AssignSpaces {
        /// The toplevel.
        toplevel: ToplevelKey,
        /// Which Spaces.
        assign: Assign,
    },
    /// Forget a window's minimise target (`quire_shell_v1`). Quire arm, shell role.
    ClearMinimiseTarget(ToplevelKey),
    /// Switch to a Space. Quire arm.
    ActivateSpace(SpaceKey),
    /// Remove a Space; its windows move to the neighbouring desktop Space. Quire arm.
    RemoveSpace(SpaceKey),
    /// Move a Space to a position on its output (0 is leftmost). Quire arm.
    MoveSpace {
        /// The Space.
        space: SpaceKey,
        /// The new position.
        position: u32,
    },
    /// Add a Space on an output; it arrives as a `QuireEvent::Space`. Quire arm.
    CreateSpace {
        /// The output.
        output: OutputName,
    },
    /// Bring an app forward and raise its windows. Quire arm.
    ActivateApp(AppKey),
    /// Hide an app. Quire arm.
    HideApp(AppKey),
    /// Show a hidden app. Quire arm.
    UnhideApp(AppKey),
    /// Hide every app but this one. Quire arm.
    HideOtherApps(AppKey),
    /// Ask an app to quit; it may refuse or ask the person first. Quire arm.
    QuitApp(AppKey),
    /// End an app's processes. Quire arm.
    ForceQuitApp(AppKey),
    /// Activate an app that has no windows; the compositor answers the shell with
    /// `ShellEvent::Reopen`. Quire arm.
    ReopenApp(AppKey),
    /// Tell the compositor what a registered layer surface is (bar, dock, ...). Quire arm,
    /// shell role.
    SetShellRole {
        /// The caller's surface.
        surface: SurfaceKey,
        /// Its role.
        role: ShellRole,
    },
    /// The person picked an app in the switcher, by the index the compositor gave. Quire arm,
    /// shell role.
    PickSwitcher(u32),
    /// The person dismissed the switcher. Quire arm, shell role.
    CancelSwitcher,
    /// Open or close an overview. Quire arm, shell role.
    Overview {
        /// Which overview.
        mode: OverviewMode,
        /// Open (`On`) or close (`Off`).
        toggle: Toggle,
    },
    /// Move every window aside, or back. Quire arm, shell role.
    ShowDesktop(Toggle),
    /// The person picked an entry of a window's menu (`ShellEvent::WindowMenu`). A dismissal
    /// sends nothing. Quire arm, shell role.
    PickWindowMenu {
        /// The window, by the identifier the menu event carried.
        window: ToplevelIdentifier,
        /// The entry.
        entry: WindowEntry,
    },
    /// The person picked a window for the other half of the Split View being chosen
    /// (`ShellEvent::SplitChoose`). Quire arm, shell role.
    PickSplit(ToplevelIdentifier),
    /// The person turned the choice down (Escape, or a click elsewhere): the first window stays
    /// alone, full screen. Quire arm, shell role.
    CancelSplit,
    /// The person moved the divider of a Split View. Quire arm, shell role.
    DivideSplit {
        /// Either window of the pair.
        window: ToplevelIdentifier,
        /// The leading (left) window's share of the width, in thousandths.
        share: u32,
    },
    /// Replaces the rectangles of `output` a window dragged in Mission Control can be dropped on
    /// to move it to a Space (version 2). Quire arm, shell role.
    SetSpaceTargets {
        /// The output.
        output: OutputName,
        /// The targets, in global logical pixels; later ones are in front.
        targets: Vec<SpaceTarget>,
    },
    /// Sets, or with `None` removes, the rectangle of `output` a window dragged in Mission Control
    /// can be dropped on to make a new Space with it (version 3). It stays when
    /// [`SetSpaceTargets`](Self::SetSpaceTargets) replaces the Space targets. Quire arm, shell
    /// role.
    SetNewSpaceTarget {
        /// The output.
        output: OutputName,
        /// The rectangle in global logical pixels; `None` removes the target.
        rect: Option<Rect>,
    },
    /// The person answered a question the compositor asked
    /// ([`ShellEvent::PermissionAsk`](crate::quire::ShellEvent::PermissionAsk)); send
    /// [`Answer::Dismissed`] when the sheet goes away without an answer (version 5). Quire arm,
    /// shell role.
    AnswerPermission {
        /// The question, by the id the ask carried.
        ask: AskId,
        /// What the person said.
        answer: Answer,
    },
    /// The person answered a question about a changed display setup
    /// ([`ShellEvent::DisplaysTrial`](crate::quire::ShellEvent::DisplaysTrial)); send
    /// [`TrialAnswer::Revert`] when the sheet goes away without an answer (version 6). Answering
    /// faster than the person could have read the question is a protocol error for the shell's
    /// connection. Quire arm, shell role.
    AnswerTrial {
        /// The trial, by the id the event carried.
        trial: TrialId,
        /// What the person said.
        answer: TrialAnswer,
    },
}

/// A place a dragged window can be dropped on a Space.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SpaceTarget {
    /// The Space the window moves to.
    pub space: SpaceId,
    /// The rectangle, in global logical pixels.
    pub rect: Rect,
}

impl SpaceTarget {
    /// A target of `rect` for `space`.
    pub fn new(space: SpaceId, rect: Rect) -> Self {
        Self { space, rect }
    }
}

/// Which protocol family carries a [`DesktopCommand`]: the one place the commands are sorted.
/// [`requires_on`](crate::requires_on) and the wire client read it, and [`CommandError`](
/// crate::CommandError) names it instead of the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CommandRoute {
    /// The plain window commands: `zwlr-foreign-toplevel-management` or, on the Quire arm,
    /// `quire_desktop_window_v1`.
    Toplevel,
    /// The minimise target: the toplevel handle, or on the Quire arm `quire_shell_v1`.
    MinimiseTarget,
    /// `ext-workspace-v1`.
    Workspace,
    /// `wlr-output-management`.
    Output,
    /// `ext-image-copy-capture`.
    Capture,
    /// A window command only the Quire arm has.
    QuireWindow,
    /// An app or Space command, on `quire_desktop_manager_v1`.
    QuireDesktop,
    /// A shell command, on `quire_shell_v1`.
    QuireShell,
}

impl DesktopCommand {
    /// Which family carries this command. Every command is named here, with no wildcard, so a
    /// new command cannot be added without choosing its route.
    pub fn route(&self) -> CommandRoute {
        match self {
            Self::Activate(_)
            | Self::Close(_)
            | Self::Minimise(_)
            | Self::Unminimise(_)
            | Self::Maximise(_)
            | Self::Unmaximise(_)
            | Self::Fullscreen { .. }
            | Self::Unfullscreen(_) => CommandRoute::Toplevel,
            Self::SetMinimiseTarget { .. } => CommandRoute::MinimiseTarget,
            Self::ActivateWorkspace(_)
            | Self::DeactivateWorkspace(_)
            | Self::AssignWorkspace { .. }
            | Self::RemoveWorkspace(_)
            | Self::CreateWorkspace { .. }
            | Self::CommitWorkspaces => CommandRoute::Workspace,
            Self::ApplyOutputs(_) | Self::TestOutputs(_) => CommandRoute::Output,
            Self::CaptureOutput(_) => CommandRoute::Capture,
            Self::Place { .. } | Self::MoveToSpace { .. } | Self::AssignSpaces { .. } => {
                CommandRoute::QuireWindow
            }
            Self::ActivateSpace(_)
            | Self::RemoveSpace(_)
            | Self::MoveSpace { .. }
            | Self::CreateSpace { .. }
            | Self::ActivateApp(_)
            | Self::HideApp(_)
            | Self::UnhideApp(_)
            | Self::HideOtherApps(_)
            | Self::QuitApp(_)
            | Self::ForceQuitApp(_)
            | Self::ReopenApp(_) => CommandRoute::QuireDesktop,
            Self::ClearMinimiseTarget(_)
            | Self::SetShellRole { .. }
            | Self::PickSwitcher(_)
            | Self::CancelSwitcher
            | Self::Overview { .. }
            | Self::ShowDesktop(_)
            | Self::PickWindowMenu { .. }
            | Self::PickSplit(_)
            | Self::CancelSplit
            | Self::DivideSplit { .. }
            | Self::SetSpaceTargets { .. }
            | Self::SetNewSpaceTarget { .. }
            | Self::AnswerPermission { .. }
            | Self::AnswerTrial { .. } => CommandRoute::QuireShell,
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
