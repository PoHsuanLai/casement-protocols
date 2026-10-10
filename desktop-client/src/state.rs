//! The whole picture.

use crate::capture::Captures;
use crate::output::Outputs;
use crate::protocol::ProtocolSet;
use crate::toplevel::Toplevels;
use crate::workspace::Workspaces;

/// Everything the compositor has said, reduced. Plain data: a consumer reads it and maps it
/// into its own model.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct DesktopState {
    /// What the compositor offers.
    pub protocols: ProtocolSet,
    /// Toplevels.
    pub toplevels: Toplevels,
    /// Workspaces and groups.
    pub workspaces: Workspaces,
    /// Output heads.
    pub outputs: Outputs,
    /// Captures in flight or waiting.
    pub captures: Captures,
}

impl DesktopState {
    /// Nothing known; `protocols` is what was offered.
    pub fn new(protocols: ProtocolSet) -> Self {
        Self {
            protocols,
            ..Self::default()
        }
    }
}
