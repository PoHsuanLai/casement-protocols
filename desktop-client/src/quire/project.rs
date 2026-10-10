//! The Quire state seen as toplevel state.

use super::set::Quire;
use super::window::QuireWindow;
use crate::toplevel::{ToplevelFlag, ToplevelFlags, Toplevels};
use casement_protocols::vocab::WindowStateKind;

/// `toplevels` with the flags, parent and output of every toplevel that has a Quire window
/// taken from that window, so a consumer that reads only toplevels sees the Quire arm's state
/// as it sees wlr's: filled is maximised, and the key window is activated.
pub(crate) fn project_toplevels(mut toplevels: Toplevels, quire: &Quire) -> Toplevels {
    for (key, window) in &quire.windows {
        if let Some(toplevel) = toplevels.0.get_mut(key) {
            toplevel.flags = flags_of(window, quire.active.key == Some(*key));
            toplevel.parent = window.parent;
            if let Some(output) = &window.output {
                toplevel.outputs = vec![output.clone()];
            }
        }
    }
    toplevels
}

/// The wlr-style flags of a window; `key` says it is the key window.
fn flags_of(window: &QuireWindow, key: bool) -> ToplevelFlags {
    let from_state = match window.state {
        WindowStateKind::Floating | WindowStateKind::Tiled => None,
        WindowStateKind::Filled => Some(ToplevelFlag::Maximized),
        WindowStateKind::FullScreen => Some(ToplevelFlag::Fullscreen),
        WindowStateKind::Minimised => Some(ToplevelFlag::Minimized),
    };
    let activated = key.then_some(ToplevelFlag::Activated);
    ToplevelFlags::of(from_state.into_iter().chain(activated))
}
