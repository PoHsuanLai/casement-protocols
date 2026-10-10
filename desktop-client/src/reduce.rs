//! The one transition.

use crate::capture::reduce_captures;
use crate::event::DesktopEvent;
use crate::output::reduce_outputs;
use crate::state::DesktopState;
use crate::toplevel::reduce_toplevels;
use crate::workspace::reduce_workspaces;

/// `state` after `event`. Pure and total: an event for something unknown changes nothing.
pub fn reduce(state: DesktopState, event: DesktopEvent) -> DesktopState {
    match event {
        DesktopEvent::Globals(protocols) => DesktopState { protocols, ..state },
        DesktopEvent::Toplevel(e) => DesktopState {
            toplevels: reduce_toplevels(state.toplevels, e),
            ..state
        },
        DesktopEvent::Workspace(e) => DesktopState {
            workspaces: reduce_workspaces(state.workspaces, e),
            ..state
        },
        DesktopEvent::Output(e) => DesktopState {
            outputs: reduce_outputs(state.outputs, e),
            ..state
        },
        DesktopEvent::Capture { id, event } => DesktopState {
            captures: reduce_captures(state.captures, id, event),
            ..state
        },
        DesktopEvent::Quire(_) => state,
    }
}
