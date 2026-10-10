//! Everything the core reduces.

use crate::capture::CaptureEvent;
use crate::keys::CaptureId;
use crate::output::OutputEvent;
use crate::protocol::ProtocolSet;
use crate::quire::QuireEvent;
use crate::toplevel::ToplevelEvent;
use crate::workspace::WorkspaceEvent;

/// One thing the compositor said, in the core's words.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DesktopEvent {
    /// The registry's offer is known (once, after the first round trip).
    Globals(ProtocolSet),
    /// A toplevel event.
    Toplevel(ToplevelEvent),
    /// A workspace event.
    Workspace(WorkspaceEvent),
    /// An output event.
    Output(OutputEvent),
    /// An event of one capture.
    Capture {
        /// Which capture.
        id: CaptureId,
        /// What happened.
        event: CaptureEvent,
    },
    /// An event of the Quire arm (P2; never produced yet).
    Quire(QuireEvent),
}
