//! Offered protocols and `supports`.

use desktop_client::capture::CaptureRequest;
use desktop_client::output::OutputConfig;
use desktop_client::{CommandKind, Rect};
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

#[test]
fn every_command_needs_something_and_has_a_distinct_kind() {
    let mut commands = toplevel_commands();
    commands.extend(workspace_commands());
    commands.push(DesktopCommand::ApplyOutputs(OutputConfig::new(vec![])));
    commands.push(DesktopCommand::TestOutputs(OutputConfig::new(vec![])));
    commands.push(DesktopCommand::CaptureOutput(CaptureRequest::new(
        OutputName::new("x"),
    )));
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
    assert_eq!(kinds.len(), 18);
}
