//! Whether the offered protocols can carry a command.

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

/// The protocols `command` needs, all of them.
pub fn requires(command: &DesktopCommand) -> &'static [Protocol] {
    use Protocol::*;
    match command {
        DesktopCommand::Activate(_)
        | DesktopCommand::Close(_)
        | DesktopCommand::Minimise(_)
        | DesktopCommand::Unminimise(_)
        | DesktopCommand::Maximise(_)
        | DesktopCommand::Unmaximise(_)
        | DesktopCommand::Fullscreen { .. }
        | DesktopCommand::Unfullscreen(_)
        | DesktopCommand::SetMinimiseTarget { .. } => &[WlrForeignToplevelManagement],
        DesktopCommand::ActivateWorkspace(_)
        | DesktopCommand::DeactivateWorkspace(_)
        | DesktopCommand::AssignWorkspace { .. }
        | DesktopCommand::RemoveWorkspace(_)
        | DesktopCommand::CreateWorkspace { .. }
        | DesktopCommand::CommitWorkspaces => &[ExtWorkspace],
        DesktopCommand::ApplyOutputs(_) | DesktopCommand::TestOutputs(_) => &[WlrOutputManagement],
        DesktopCommand::CaptureOutput(_) => &[ExtImageCopyCapture, ExtOutputImageCaptureSource],
    }
}

/// The first protocol `command` needs that `offered` lacks.
pub fn missing(offered: &ProtocolSet, command: &DesktopCommand) -> Option<Protocol> {
    requires(command)
        .iter()
        .copied()
        .find(|p| !offered.contains(*p))
}

/// Whether `offered` carries `command`. The compositor may still refuse at run time (a
/// workspace that lacks the ability, a closed toplevel); this answers the protocol question.
pub fn supports(offered: &ProtocolSet, command: &DesktopCommand) -> Supports {
    match missing(offered, command) {
        None => Supports::Yes,
        Some(_) => Supports::No,
    }
}
