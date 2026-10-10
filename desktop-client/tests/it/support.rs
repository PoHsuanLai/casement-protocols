//! Shared builders.

use desktop_client::capture::CaptureEvent;
use desktop_client::output::{HeadChange, ModeChange, OutputEvent};
use desktop_client::toplevel::{ToplevelChange, ToplevelEvent};
use desktop_client::workspace::{WorkspaceChange, WorkspaceEvent};
use desktop_client::{
    CaptureId, DesktopEvent, DesktopState, GroupKey, HeadKey, ModeKey, ToplevelKey, WorkspaceKey,
    reduce,
};

pub fn tk(n: u64) -> ToplevelKey {
    ToplevelKey::new(n)
}

pub fn wk(n: u64) -> WorkspaceKey {
    WorkspaceKey::new(n)
}

pub fn gk(n: u64) -> GroupKey {
    GroupKey::new(n)
}

pub fn hk(n: u64) -> HeadKey {
    HeadKey::new(n)
}

pub fn mk(n: u64) -> ModeKey {
    ModeKey::new(n)
}

pub fn cid(n: u64) -> CaptureId {
    CaptureId::new(n)
}

/// A state after `events`, from nothing.
pub fn run(events: impl IntoIterator<Item = DesktopEvent>) -> DesktopState {
    events
        .into_iter()
        .fold(DesktopState::default(), |state, event| reduce(state, event))
}

pub fn top(event: ToplevelEvent) -> DesktopEvent {
    DesktopEvent::Toplevel(event)
}

pub fn top_change(key: ToplevelKey, change: ToplevelChange) -> DesktopEvent {
    top(ToplevelEvent::Changed { key, change })
}

pub fn ws(event: WorkspaceEvent) -> DesktopEvent {
    DesktopEvent::Workspace(event)
}

pub fn ws_change(key: WorkspaceKey, change: WorkspaceChange) -> DesktopEvent {
    ws(WorkspaceEvent::Changed { key, change })
}

pub fn out(event: OutputEvent) -> DesktopEvent {
    DesktopEvent::Output(event)
}

pub fn head_change(key: HeadKey, change: HeadChange) -> DesktopEvent {
    out(OutputEvent::Head { key, change })
}

pub fn mode_change(mode: ModeKey, change: ModeChange) -> DesktopEvent {
    out(OutputEvent::Mode { mode, change })
}

pub fn cap(id: CaptureId, event: CaptureEvent) -> DesktopEvent {
    DesktopEvent::Capture { id, event }
}
