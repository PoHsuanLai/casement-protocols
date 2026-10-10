//! The fake desktop.

use desktop_client::capture::{CaptureEvent, CapturePhase, CaptureRequest, ShmFormat};
use desktop_client::fake::{Advance, FakeDesktop};
use desktop_client::toplevel::{ToplevelChange, ToplevelEvent};
use desktop_client::{
    CommandError, Desktop, DesktopCommand, DesktopEvent, OutputName, Protocol, ProtocolSet, Size,
    Ticket,
};

use crate::support::{cap, cid, tk, top, top_change};

fn offering(protocols: &[Protocol]) -> FakeDesktop {
    let set = protocols
        .iter()
        .fold(ProtocolSet::empty(), |set, p| set.with(*p));
    FakeDesktop::new(set)
}

#[test]
fn scripted_events_reach_the_state_one_advance_at_a_time() {
    let mut desktop = FakeDesktop::new(ProtocolSet::empty());
    desktop.script(top(ToplevelEvent::Opened(tk(1))));
    desktop.script(top_change(tk(1), ToplevelChange::Title("hi".into())));
    desktop.script(top(ToplevelEvent::Done(tk(1))));

    assert_eq!(desktop.advance(), Advance::Applied);
    assert_eq!(desktop.state().toplevels.announced().count(), 0);
    desktop.advance_all();
    assert_eq!(desktop.state().toplevels.announced().count(), 1);
    assert_eq!(desktop.state().toplevels.get(tk(1)).unwrap().title, "hi");
    assert_eq!(desktop.advance(), Advance::Idle);
}

#[test]
fn drain_events_returns_what_was_applied_once() {
    let mut desktop = FakeDesktop::new(ProtocolSet::empty());
    desktop.script(top(ToplevelEvent::Opened(tk(1))));
    desktop.script(top(ToplevelEvent::Done(tk(1))));
    desktop.advance_all();
    let drained = desktop.drain_events();
    assert_eq!(drained.len(), 2);
    assert!(matches!(drained[0], DesktopEvent::Toplevel(_)));
    assert!(desktop.drain_events().is_empty());
}

#[test]
fn commands_are_checked_against_the_offer_and_recorded() {
    let mut desktop = offering(&[Protocol::WlrForeignToplevelManagement]);
    assert_eq!(desktop.send(DesktopCommand::Close(tk(1))), Ok(Ticket::Sent));
    assert_eq!(desktop.sent(), &[DesktopCommand::Close(tk(1))]);

    let refused = desktop.send(DesktopCommand::CommitWorkspaces);
    assert!(matches!(
        refused,
        Err(CommandError::Unsupported {
            needs: Protocol::ExtWorkspace,
            ..
        })
    ));
    assert_eq!(desktop.sent().len(), 1);
    assert_eq!(desktop.take_sent().len(), 1);
    assert!(desktop.sent().is_empty());
}

#[test]
fn captures_get_ids_and_finish_through_the_reducer() {
    let mut desktop = offering(&[
        Protocol::ExtImageCopyCapture,
        Protocol::ExtOutputImageCaptureSource,
    ]);
    let request = CaptureRequest::new(OutputName::new("DP-1"));
    let first = desktop
        .send(DesktopCommand::CaptureOutput(request.clone()))
        .unwrap();
    let second = desktop
        .send(DesktopCommand::CaptureOutput(request))
        .unwrap();
    let (Ticket::Capture(a), Ticket::Capture(b)) = (first, second) else {
        panic!("expected capture tickets");
    };
    assert_ne!(a, b);

    for event in [
        CaptureEvent::BufferSize(Size::new(1, 1)),
        CaptureEvent::Format(ShmFormat::ARGB8888),
        CaptureEvent::ConstraintsDone,
        CaptureEvent::Ready {
            stride: 4,
            pixels: vec![0, 0, 0, 255],
        },
    ] {
        desktop.script(cap(a, event));
    }
    desktop.advance_all();
    assert!(matches!(
        desktop.state().captures.get(a).unwrap().phase,
        CapturePhase::Ready(_)
    ));
    assert!(desktop.state().captures.get(cid(1000)).is_none());
}

#[test]
fn configurations_get_their_own_tickets() {
    use desktop_client::output::OutputConfig;
    let mut desktop = offering(&[Protocol::WlrOutputManagement]);
    let ticket = desktop
        .send(DesktopCommand::TestOutputs(OutputConfig::new(vec![])))
        .unwrap();
    assert!(matches!(ticket, Ticket::Config(_)));
}
