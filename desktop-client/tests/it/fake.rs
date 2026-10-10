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

// The Quire arm.

fn quire_fake() -> FakeDesktop {
    offering(&[
        Protocol::ExtForeignToplevelList,
        Protocol::QuireDesktop,
        Protocol::QuireShell,
    ])
}

/// Scripts a toplevel with an identifier, its Quire window, an app and a Space.
fn populate(desktop: &mut FakeDesktop) {
    use desktop_client::quire::{QuireEvent, SpaceEvent, WindowEvent};
    use desktop_client::{AppKey, SpaceKey, ToplevelIdentifier};
    for event in [
        top(ToplevelEvent::Opened(tk(1))),
        top_change(
            tk(1),
            ToplevelChange::Identifier(ToplevelIdentifier::new("w-1")),
        ),
        top(ToplevelEvent::Done(tk(1))),
        DesktopEvent::Quire(QuireEvent::Window(WindowEvent::Opened(tk(1)))),
        DesktopEvent::Quire(QuireEvent::App(desktop_client::quire::AppEvent::Opened(
            AppKey::new(1),
        ))),
        DesktopEvent::Quire(QuireEvent::Space(SpaceEvent::Opened(SpaceKey::new(1)))),
    ] {
        desktop.script(event);
    }
    desktop.advance_all();
}

#[test]
fn the_quire_commands_are_recorded_when_their_targets_exist() {
    use casement_protocols::vocab::{Assign, PlaceTarget};
    use desktop_client::{AppKey, SpaceKey};
    let mut desktop = quire_fake();
    populate(&mut desktop);
    let commands = [
        DesktopCommand::Place {
            toplevel: tk(1),
            target: PlaceTarget::TopRight,
        },
        DesktopCommand::MoveToSpace {
            toplevel: tk(1),
            space: SpaceKey::new(1),
        },
        DesktopCommand::AssignSpaces {
            toplevel: tk(1),
            assign: Assign::ThisSpace,
        },
        DesktopCommand::ActivateSpace(SpaceKey::new(1)),
        DesktopCommand::QuitApp(AppKey::new(1)),
        DesktopCommand::CreateSpace {
            output: OutputName::new("DP-1"),
        },
        DesktopCommand::Close(tk(1)),
    ];
    for command in commands.clone() {
        assert_eq!(desktop.send(command), Ok(Ticket::Sent));
    }
    assert_eq!(desktop.sent(), &commands);
}

#[test]
fn a_quire_command_naming_nothing_is_refused_and_not_recorded() {
    use casement_protocols::vocab::PlaceTarget;
    use desktop_client::{AppKey, SpaceKey};
    let mut desktop = quire_fake();
    populate(&mut desktop);
    assert_eq!(
        desktop.send(DesktopCommand::Place {
            toplevel: tk(9),
            target: PlaceTarget::Fill,
        }),
        Err(CommandError::UnknownToplevel(tk(9)))
    );
    assert_eq!(
        desktop.send(DesktopCommand::ActivateSpace(SpaceKey::new(9))),
        Err(CommandError::UnknownSpace(SpaceKey::new(9)))
    );
    assert_eq!(
        desktop.send(DesktopCommand::HideApp(AppKey::new(9))),
        Err(CommandError::UnknownApp(AppKey::new(9)))
    );
    assert!(desktop.sent().is_empty());
}

#[test]
fn a_quire_command_on_a_wlr_only_compositor_is_unsupported() {
    let mut desktop = offering(&[Protocol::WlrForeignToplevelManagement]);
    let refused = desktop.send(DesktopCommand::QuitApp(desktop_client::AppKey::new(1)));
    assert!(matches!(
        refused,
        Err(CommandError::Unsupported {
            needs: Protocol::QuireDesktop,
            ..
        })
    ));
}

#[test]
fn shell_commands_need_the_shell_role() {
    use casement_protocols::vocab::Toggle;
    let command = DesktopCommand::ShowDesktop(Toggle::On);
    let mut skipped = quire_fake();
    assert!(matches!(
        skipped.send(command.clone()),
        Err(CommandError::NotBound {
            needs: Protocol::QuireShell,
            ..
        })
    ));
    let mut shell = FakeDesktop::with_config(
        ProtocolSet::empty().with(Protocol::QuireShell),
        desktop_client::DesktopConfig::holding_shell(),
    );
    assert_eq!(shell.send(command.clone()), Ok(Ticket::Sent));
    assert_eq!(shell.sent(), &[command]);
}

#[test]
fn the_minimise_target_needs_the_window_identifier_on_the_quire_arm() {
    use desktop_client::{Rect, SurfaceKey};
    let target = |toplevel| DesktopCommand::SetMinimiseTarget {
        toplevel,
        surface: SurfaceKey::new(1),
        rect: Rect::new(0, 0, 40, 40),
    };
    let mut desktop = FakeDesktop::with_config(
        ProtocolSet::empty()
            .with(Protocol::ExtForeignToplevelList)
            .with(Protocol::QuireDesktop)
            .with(Protocol::QuireShell),
        desktop_client::DesktopConfig::holding_shell(),
    );
    desktop.script(top(ToplevelEvent::Opened(tk(1))));
    desktop.script(top(ToplevelEvent::Done(tk(1))));
    desktop.advance_all();
    assert_eq!(
        desktop.send(target(tk(1))),
        Err(CommandError::NoIdentifier(tk(1)))
    );
    assert_eq!(
        desktop.send(target(tk(2))),
        Err(CommandError::UnknownToplevel(tk(2)))
    );
    desktop.script(top_change(
        tk(1),
        ToplevelChange::Identifier(desktop_client::ToplevelIdentifier::new("w-1")),
    ));
    desktop.advance_all();
    assert_eq!(desktop.send(target(tk(1))), Ok(Ticket::Sent));
}

#[test]
fn quire_events_reduce_through_the_fake_like_any_other() {
    use desktop_client::quire::{QuireEvent, ShellEvent};
    let mut desktop = quire_fake();
    desktop.script(DesktopEvent::Quire(QuireEvent::Shell(
        ShellEvent::SwitcherShown { selected: 3 },
    )));
    desktop.advance_all();
    assert_eq!(desktop.state().quire.shell().switcher.selected(), Some(3));
    assert_eq!(desktop.drain_events().len(), 1);
}
