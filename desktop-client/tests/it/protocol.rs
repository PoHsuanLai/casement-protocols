//! Offered protocols and `supports`.

use casement_protocols::vocab::{Assign, OverviewMode, PlaceTarget, ShellRole, Toggle};
use desktop_client::capture::CaptureRequest;
use desktop_client::output::OutputConfig;
use desktop_client::{AppKey, CommandKind, DesktopBackend, Rect, SpaceKey, requires_on};
use desktop_client::{
    DesktopCommand, GroupKey, OutputName, Protocol, ProtocolSet, Supports, SurfaceKey, ToplevelKey,
    WorkspaceKey, missing, requires, supports,
};

fn toplevel_commands() -> Vec<DesktopCommand> {
    let t = ToplevelKey::new(1);
    vec![
        DesktopCommand::Activate(t),
        DesktopCommand::Close(t),
        DesktopCommand::Minimise(t),
        DesktopCommand::Unminimise(t),
        DesktopCommand::Maximise(t),
        DesktopCommand::Unmaximise(t),
        DesktopCommand::Fullscreen {
            toplevel: t,
            output: None,
        },
        DesktopCommand::Unfullscreen(t),
        DesktopCommand::SetMinimiseTarget {
            toplevel: t,
            surface: SurfaceKey::new(1),
            rect: Rect::new(0, 0, 10, 10),
        },
    ]
}

fn workspace_commands() -> Vec<DesktopCommand> {
    let w = WorkspaceKey::new(1);
    let g = GroupKey::new(1);
    vec![
        DesktopCommand::ActivateWorkspace(w),
        DesktopCommand::DeactivateWorkspace(w),
        DesktopCommand::AssignWorkspace {
            workspace: w,
            group: g,
        },
        DesktopCommand::RemoveWorkspace(w),
        DesktopCommand::CreateWorkspace {
            group: g,
            name: "x".into(),
        },
        DesktopCommand::CommitWorkspaces,
    ]
}

#[test]
fn interfaces_round_trip_and_unknown_ones_are_ignored() {
    for protocol in Protocol::ALL {
        assert_eq!(
            Protocol::from_interface(protocol.interface()),
            Some(protocol)
        );
    }
    assert_eq!(Protocol::from_interface("wl_compositor"), None);
    let set = ProtocolSet::from_interfaces(["wl_compositor", "ext_workspace_manager_v1"]);
    assert!(set.contains(Protocol::ExtWorkspace));
    assert_eq!(set.iter().count(), 1);
    assert_eq!(
        ProtocolSet::empty().with(Protocol::WlrOutputManagement),
        ProtocolSet::from_interfaces(["zwlr_output_manager_v1"])
    );
}

#[test]
fn toplevel_commands_need_the_wlr_manager_only() {
    let wlr = ProtocolSet::empty().with(Protocol::WlrForeignToplevelManagement);
    let list_only = ProtocolSet::empty().with(Protocol::ExtForeignToplevelList);
    for command in toplevel_commands() {
        assert_eq!(
            supports(&wlr, &command),
            Supports::Yes,
            "{:?}",
            command.kind()
        );
        assert_eq!(
            supports(&list_only, &command),
            Supports::No,
            "{:?}",
            command.kind()
        );
        assert_eq!(
            missing(&list_only, &command),
            Some(Protocol::WlrForeignToplevelManagement)
        );
    }
}

#[test]
fn workspace_commands_need_ext_workspace() {
    let offered = ProtocolSet::empty().with(Protocol::ExtWorkspace);
    for command in workspace_commands() {
        assert_eq!(
            supports(&offered, &command),
            Supports::Yes,
            "{:?}",
            command.kind()
        );
        assert_eq!(supports(&ProtocolSet::empty(), &command), Supports::No);
    }
}

#[test]
fn output_commands_need_wlr_output_management() {
    let offered = ProtocolSet::empty().with(Protocol::WlrOutputManagement);
    for command in [
        DesktopCommand::ApplyOutputs(OutputConfig::new(vec![])),
        DesktopCommand::TestOutputs(OutputConfig::new(vec![])),
    ] {
        assert_eq!(supports(&offered, &command), Supports::Yes);
        assert_eq!(supports(&ProtocolSet::empty(), &command), Supports::No);
    }
}

#[test]
fn capture_needs_both_the_copy_manager_and_the_output_source() {
    let command = DesktopCommand::CaptureOutput(CaptureRequest::new(OutputName::new("DP-1")));
    let copy = ProtocolSet::empty().with(Protocol::ExtImageCopyCapture);
    let both = copy.clone().with(Protocol::ExtOutputImageCaptureSource);
    assert_eq!(supports(&copy, &command), Supports::No);
    assert_eq!(
        missing(&copy, &command),
        Some(Protocol::ExtOutputImageCaptureSource)
    );
    assert_eq!(supports(&both, &command), Supports::Yes);
    assert_eq!(requires(&command).len(), 2);
}

fn offering(protocols: &[Protocol]) -> ProtocolSet {
    protocols
        .iter()
        .fold(ProtocolSet::empty(), |set, p| set.with(*p))
}

/// The commands only the Quire arm carries, each with the protocols it needs.
fn quire_only() -> Vec<(DesktopCommand, Vec<Protocol>)> {
    let t = ToplevelKey::new(1);
    let s = SpaceKey::new(1);
    let a = AppKey::new(1);
    let window = vec![Protocol::QuireDesktop, Protocol::ExtForeignToplevelList];
    let desktop = vec![Protocol::QuireDesktop];
    let shell = vec![Protocol::QuireShell];
    vec![
        (
            DesktopCommand::Place {
                toplevel: t,
                target: PlaceTarget::Left,
            },
            window.clone(),
        ),
        (
            DesktopCommand::MoveToSpace {
                toplevel: t,
                space: s,
            },
            window.clone(),
        ),
        (
            DesktopCommand::AssignSpaces {
                toplevel: t,
                assign: Assign::AllSpaces,
            },
            window,
        ),
        (DesktopCommand::ActivateSpace(s), desktop.clone()),
        (DesktopCommand::RemoveSpace(s), desktop.clone()),
        (
            DesktopCommand::MoveSpace {
                space: s,
                position: 2,
            },
            desktop.clone(),
        ),
        (
            DesktopCommand::CreateSpace {
                output: OutputName::new("DP-1"),
            },
            desktop.clone(),
        ),
        (DesktopCommand::ActivateApp(a), desktop.clone()),
        (DesktopCommand::HideApp(a), desktop.clone()),
        (DesktopCommand::UnhideApp(a), desktop.clone()),
        (DesktopCommand::HideOtherApps(a), desktop.clone()),
        (DesktopCommand::QuitApp(a), desktop.clone()),
        (DesktopCommand::ForceQuitApp(a), desktop.clone()),
        (DesktopCommand::ReopenApp(a), desktop),
        (DesktopCommand::ClearMinimiseTarget(t), shell.clone()),
        (
            DesktopCommand::SetShellRole {
                surface: SurfaceKey::new(1),
                role: ShellRole::Dock,
            },
            shell.clone(),
        ),
        (DesktopCommand::PickSwitcher(0), shell.clone()),
        (DesktopCommand::CancelSwitcher, shell.clone()),
        (
            DesktopCommand::Overview {
                mode: OverviewMode::MissionControl,
                toggle: Toggle::On,
            },
            shell.clone(),
        ),
        (DesktopCommand::ShowDesktop(Toggle::Off), shell),
    ]
}

#[test]
fn quire_commands_need_exactly_their_quire_protocols() {
    for (command, needs) in quire_only() {
        assert_eq!(requires(&command), needs.as_slice(), "{:?}", command.kind());
        let offered = needs
            .iter()
            .fold(ProtocolSet::empty(), |set, p| set.with(*p));
        assert_eq!(
            supports(&offered, &command),
            Supports::Yes,
            "{:?}",
            command.kind()
        );
        assert_eq!(missing(&offered, &command), None);
        for dropped in &needs {
            let partial = needs
                .iter()
                .filter(|p| *p != dropped)
                .fold(ProtocolSet::empty(), |set, p| set.with(*p));
            assert_eq!(supports(&partial, &command), Supports::No);
            assert_eq!(missing(&partial, &command), Some(*dropped));
        }
    }
}

#[test]
fn wlr_alone_cannot_do_what_only_quire_can() {
    let wlr = offering(&[
        Protocol::WlrForeignToplevelManagement,
        Protocol::ExtForeignToplevelList,
    ]);
    for (command, _) in quire_only() {
        assert_eq!(
            supports(&wlr, &command),
            Supports::No,
            "{:?}",
            command.kind()
        );
    }
}

#[test]
fn on_the_quire_arm_the_window_commands_go_to_quire_not_wlr() {
    let quire = offering(&[Protocol::QuireDesktop, Protocol::ExtForeignToplevelList]);
    let wlr_too = quire.clone().with(Protocol::WlrForeignToplevelManagement);
    for command in toplevel_commands() {
        let kind = command.kind();
        if kind == CommandKind::SetMinimiseTarget {
            // The minimise target is the shell's: quire_shell_v1.
            assert_eq!(missing(&quire, &command), Some(Protocol::QuireShell));
            assert_eq!(
                supports(&quire.clone().with(Protocol::QuireShell), &command),
                Supports::Yes
            );
        } else {
            assert_eq!(supports(&quire, &command), Supports::Yes, "{kind:?}");
            assert_eq!(supports(&wlr_too, &command), Supports::Yes, "{kind:?}");
        }
        assert_eq!(
            requires_on(Some(DesktopBackend::Wlr), &command),
            &[Protocol::WlrForeignToplevelManagement],
            "{kind:?} on the wlr arm"
        );
    }
}

#[test]
fn quire_without_the_ext_list_falls_back_to_wlr_for_the_window_commands() {
    let offered = offering(&[
        Protocol::QuireDesktop,
        Protocol::WlrForeignToplevelManagement,
    ]);
    assert_eq!(
        supports(&offered, &DesktopCommand::Close(ToplevelKey::new(1))),
        Supports::Yes
    );
    let only_quire = offering(&[Protocol::QuireDesktop]);
    assert_eq!(
        missing(&only_quire, &DesktopCommand::Close(ToplevelKey::new(1))),
        Some(Protocol::WlrForeignToplevelManagement)
    );
}

#[test]
fn every_command_has_a_distinct_kind_and_needs_something() {
    let mut commands = toplevel_commands();
    commands.extend(workspace_commands());
    commands.push(DesktopCommand::ApplyOutputs(OutputConfig::new(vec![])));
    commands.push(DesktopCommand::TestOutputs(OutputConfig::new(vec![])));
    commands.push(DesktopCommand::CaptureOutput(CaptureRequest::new(
        OutputName::new("x"),
    )));
    commands.extend(quire_only().into_iter().map(|(c, _)| c));
    let mut kinds: Vec<CommandKind> = Vec::new();
    for command in &commands {
        assert!(!requires(command).is_empty(), "{:?}", command.kind());
        assert!(
            !kinds.contains(&command.kind()),
            "{:?} twice",
            command.kind()
        );
        kinds.push(command.kind());
    }
    assert_eq!(kinds.len(), 38);
}
