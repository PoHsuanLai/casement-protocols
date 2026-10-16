//! `quire_shell_v1` dispatch: the shell's system events.

use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::{Connection, Dispatch, QueueHandle};

use casement_protocols::client::quire_shell_v1::quire_shell_v1::{self, QuireShellV1};
use casement_protocols::vocab::{AskId, MacMods, TrialId};

use super::wire::{Wire, raw, vocab};
use crate::event::DesktopEvent;
use crate::geometry::{Point, Rect};
use crate::keys::SurfaceKey;
use crate::names::{AppId, SpaceId, ToplevelIdentifier};
use crate::quire::{DisplayTrial, PermissionAsk, QuireEvent, ShellEvent};

/// The key the caller registered `surface` under, if it did.
fn key_of(wire: &Wire, surface: &WlSurface) -> Option<SurfaceKey> {
    wire.handles
        .surfaces
        .iter()
        .find_map(|(key, s)| (s == surface).then_some(*key))
}

/// The state-event for a shell event; `None` for a code the vocab does not know.
fn translate(wire: &Wire, event: quire_shell_v1::Event) -> Option<ShellEvent> {
    use quire_shell_v1::Event;
    Some(match event {
        Event::Modifiers { surface, mods } => ShellEvent::Modifiers {
            surface: key_of(wire, &surface),
            mods: MacMods(raw(mods)),
        },
        Event::KeyGesture { gesture } => ShellEvent::KeyGesture(vocab(gesture)?),
        Event::SwitcherApp { index, app_key } => ShellEvent::SwitcherApp {
            index,
            app: AppId::new(app_key),
        },
        Event::SwitcherShown { selected } => ShellEvent::SwitcherShown { selected },
        Event::SwitcherSelected { index } => ShellEvent::SwitcherSelected(index),
        Event::SwitcherHidden => ShellEvent::SwitcherHidden,
        Event::OverviewState {
            mode,
            state,
            progress,
        } => ShellEvent::Overview {
            mode: vocab(mode)?,
            state: vocab(state)?,
            progress,
        },
        Event::OverviewSlot {
            toplevel,
            x,
            y,
            width,
            height,
        } => ShellEvent::OverviewSlot {
            window: ToplevelIdentifier::new(toplevel),
            rect: Rect::new(x, y, width, height),
        },
        Event::OverviewSpace {
            space_id,
            x,
            y,
            width,
            height,
        } => ShellEvent::OverviewSpace {
            space: SpaceId::new(space_id),
            rect: Rect::new(x, y, width, height),
        },
        Event::OverviewDone => ShellEvent::OverviewDone,
        Event::HotCorner { corner } => ShellEvent::HotCorner(vocab(corner)?),
        Event::Reveal { edge, shown } => ShellEvent::Reveal {
            edge: vocab(edge)?,
            shown: vocab(shown)?,
        },
        Event::MenuFlash { app_key, item } => ShellEvent::MenuFlash {
            app: AppId::new(app_key),
            item,
        },
        Event::Bell { app_key } => ShellEvent::Bell(AppId::new(app_key)),
        Event::Reopen { app_key } => ShellEvent::Reopen(AppId::new(app_key)),
        Event::WindowMenu {
            toplevel,
            x,
            y,
            enabled,
        } => ShellEvent::WindowMenu {
            window: ToplevelIdentifier::new(toplevel),
            at: Point::new(x, y),
            enabled,
        },
        Event::SplitChoose { first, side } => ShellEvent::SplitChoose {
            first: ToplevelIdentifier::new(first),
            side: vocab(side)?,
        },
        Event::SplitCandidate { toplevel } => {
            ShellEvent::SplitCandidate(ToplevelIdentifier::new(toplevel))
        }
        Event::SplitChoiceEnd { first } => {
            ShellEvent::SplitChoiceEnd(ToplevelIdentifier::new(first))
        }
        Event::OverviewOutput { output, shown } => ShellEvent::OverviewOutput {
            output: wire.output_name(&output)?,
            shown: vocab(shown)?,
        },
        Event::DropTarget { output, space_id } => ShellEvent::DropTarget {
            output: wire.output_name(&output)?,
            space: space_id.map(SpaceId::new),
        },
        Event::DropTargetNew { output, over } => ShellEvent::DropTargetNew {
            output: wire.output_name(&output)?,
            over: vocab(over)?,
        },
        Event::WindowDragBegin { output, toplevel } => ShellEvent::WindowDragBegin {
            output: wire.output_name(&output)?,
            window: ToplevelIdentifier::new(toplevel),
        },
        Event::WindowDragEnd {
            output,
            toplevel,
            outcome,
        } => ShellEvent::WindowDragEnd {
            output: wire.output_name(&output)?,
            window: ToplevelIdentifier::new(toplevel),
            outcome: vocab(outcome)?,
        },
        Event::PermissionAsk {
            ask,
            power,
            subject_kind,
            subject,
            memory,
        } => ShellEvent::PermissionAsk(PermissionAsk::new(
            AskId::new(ask),
            vocab(power)?,
            vocab(subject_kind)?,
            subject,
            vocab(memory)?,
        )),
        Event::PermissionWithdraw { ask } => ShellEvent::PermissionWithdraw(AskId::new(ask)),
        Event::DisplaysTrial { trial, seconds } => {
            ShellEvent::DisplaysTrial(DisplayTrial::new(TrialId::new(trial), seconds))
        }
        Event::DisplaysTrialEnd { trial } => ShellEvent::DisplaysTrialEnd(TrialId::new(trial)),
        Event::Killed => ShellEvent::Killed,
        _ => return None,
    })
}

/// A shell event, reduced like any other.
pub(super) fn on_shell(wire: &mut Wire, event: quire_shell_v1::Event) {
    if let Some(event) = translate(wire, event) {
        wire.emit(DesktopEvent::Quire(QuireEvent::Shell(event)));
    }
}

impl Dispatch<QuireShellV1, ()> for Wire {
    fn event(
        state: &mut Self,
        _proxy: &QuireShellV1,
        event: quire_shell_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_shell(state, event);
    }
}
