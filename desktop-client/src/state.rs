//! The whole picture.

use crate::backend::{DesktopBackend, backend_for};
use crate::capture::Captures;
use crate::keys::SpaceKey;
use crate::output::Outputs;
use crate::protocol::ProtocolSet;
use crate::quire::Quire;
use crate::toplevel::Toplevels;
use crate::workspace::Workspaces;
use casement_protocols::vocab::Shown;

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
    /// The Quire arm: apps, windows, Spaces and the shell's state. Empty unless the compositor
    /// offers `quire_desktop_v1`.
    pub quire: Quire,
}

impl DesktopState {
    /// Nothing known; `protocols` is what was offered.
    pub fn new(protocols: ProtocolSet) -> Self {
        Self {
            protocols,
            ..Self::default()
        }
    }

    /// The arm the offer chose (see [`backend_for`]).
    pub fn backend(&self) -> Option<DesktopBackend> {
        backend_for(&self.protocols)
    }

    /// Whether a Space is the one its output shows. The Space protocol carries no active flag:
    /// the Space's id is its ext-workspace `id`, and that workspace's `active` flag says it.
    /// `Hidden` for an unknown Space or one with no workspace yet.
    ///
    /// ```
    /// use desktop_client::DesktopState;
    /// use desktop_client::SpaceKey;
    /// use casement_protocols::vocab::Shown;
    ///
    /// assert_eq!(DesktopState::default().space_shown(SpaceKey::new(1)), Shown::Hidden);
    /// ```
    pub fn space_shown(&self, space: SpaceKey) -> Shown {
        let active = self
            .quire
            .space(space)
            .and_then(|s| s.id.as_ref())
            .is_some_and(|id| {
                self.workspaces
                    .active()
                    .any(|w| w.id.as_deref() == Some(id.as_str()))
            });
        if active { Shown::Shown } else { Shown::Hidden }
    }
}
