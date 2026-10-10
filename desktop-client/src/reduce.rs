//! The one transition.

use crate::capture::reduce_captures;
use crate::event::DesktopEvent;
use crate::output::reduce_outputs;
use crate::quire::{QuireEvent, project_toplevels, reduce_quire};
use crate::state::DesktopState;
use crate::toplevel::{ToplevelEvent, reduce_toplevels};
use crate::workspace::reduce_workspaces;

/// `state` after `event`. Pure and total: an event for something unknown changes nothing.
pub fn reduce(state: DesktopState, event: DesktopEvent) -> DesktopState {
    match event {
        DesktopEvent::Globals(protocols) => DesktopState { protocols, ..state },
        DesktopEvent::Toplevel(e) => toplevel(state, e),
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
        DesktopEvent::Quire(e) => quire(state, e),
    }
}

/// A toplevel event; a toplevel that closes takes its Quire window with it.
fn toplevel(state: DesktopState, event: ToplevelEvent) -> DesktopState {
    let mut quire = state.quire;
    if let ToplevelEvent::Closed(key) = &event {
        quire.forget_window(*key);
    }
    DesktopState {
        toplevels: reduce_toplevels(state.toplevels, event),
        quire,
        ..state
    }
}

/// A Quire event; the windows' state is then projected onto the toplevels.
fn quire(state: DesktopState, event: QuireEvent) -> DesktopState {
    let quire = reduce_quire(state.quire, event);
    DesktopState {
        toplevels: project_toplevels(state.toplevels, &quire),
        quire,
        ..state
    }
}
