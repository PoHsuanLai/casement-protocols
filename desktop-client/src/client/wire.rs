//! The state the Wayland dispatchers write into.

use super::handles::Handles;
use crate::event::DesktopEvent;
use crate::state::DesktopState;

/// Everything the dispatch impls touch: the reduced state, the events not yet drained and the
/// proxies behind the keys.
#[derive(Debug)]
pub(super) struct Wire {
    pub(super) state: DesktopState,
    pub(super) events: Vec<DesktopEvent>,
    pub(super) handles: Handles,
}

impl Wire {
    /// Reduces `event` into the state and keeps it for draining.
    pub(super) fn emit(&mut self, event: DesktopEvent) {
        let state = std::mem::take(&mut self.state);
        self.state = crate::reduce(state, event.clone());
        self.events.push(event);
    }
}
