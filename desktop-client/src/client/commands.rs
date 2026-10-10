//! Sending a command.

use wayland_client::Proxy;
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_handle_v1::{
    self, ZwlrForeignToplevelHandleV1,
};

use super::capture;
use super::error::WireError;
use super::output;
use super::wire::Wire;
use crate::command::{DesktopCommand, Ticket};
use crate::error::CommandError;
use crate::geometry::Rect;
use crate::keys::{GroupKey, SurfaceKey, ToplevelKey, WorkspaceKey};
use crate::names::OutputName;
use crate::output::ConfigPurpose;
use crate::supports::missing;

/// Checks `command` with `supports`, resolves its keys against the handles and sends it.
/// A key with no handle is `CommandError::Unknown…`; workspace requests are staged by the
/// compositor until `CommitWorkspaces`; a configuration is validated against the state first
/// and completed with every unmentioned enabled head.
pub(super) fn send(wire: &mut Wire, command: DesktopCommand) -> Result<Ticket, WireError> {
    if let Some(needs) = missing(&wire.state.protocols, &command) {
        return Err(CommandError::Unsupported {
            command: command.kind(),
            needs,
        }
        .into());
    }
    match command {
        DesktopCommand::ApplyOutputs(config) => Ok(Ticket::Config(output::submit(
            wire,
            &config,
            ConfigPurpose::Apply,
        )?)),
        DesktopCommand::TestOutputs(config) => Ok(Ticket::Config(output::submit(
            wire,
            &config,
            ConfigPurpose::Test,
        )?)),
        DesktopCommand::CaptureOutput(request) => {
            Ok(Ticket::Capture(capture::start(wire, &request)?))
        }
        DesktopCommand::ActivateWorkspace(_)
        | DesktopCommand::DeactivateWorkspace(_)
        | DesktopCommand::AssignWorkspace { .. }
        | DesktopCommand::RemoveWorkspace(_)
        | DesktopCommand::CreateWorkspace { .. }
        | DesktopCommand::CommitWorkspaces => {
            workspace(wire, command)?;
            Ok(Ticket::Sent)
        }
        other => {
            toplevel(wire, other)?;
            Ok(Ticket::Sent)
        }
    }
}

fn managed(wire: &Wire, key: ToplevelKey) -> Result<&ZwlrForeignToplevelHandleV1, CommandError> {
    wire.handles
        .managed
        .get(&key)
        .ok_or(CommandError::UnknownToplevel(key))
}

fn output_of(wire: &Wire, name: &OutputName) -> Result<&WlOutput, CommandError> {
    wire.handles
        .outputs
        .get(name)
        .ok_or_else(|| CommandError::UnknownOutput(name.clone()))
}

fn seat(wire: &Wire) -> Result<&WlSeat, CommandError> {
    wire.handles.seat.as_ref().ok_or(CommandError::NoSeat)
}

fn surface(
    wire: &Wire,
    key: SurfaceKey,
) -> Result<&wayland_client::protocol::wl_surface::WlSurface, CommandError> {
    wire.handles
        .surfaces
        .get(&key)
        .ok_or(CommandError::UnknownSurface(key))
}

/// The toplevel commands (`zwlr-foreign-toplevel-management-v1`).
fn toplevel(wire: &Wire, command: DesktopCommand) -> Result<(), CommandError> {
    match command {
        DesktopCommand::Activate(key) => managed(wire, key)?.activate(seat(wire)?),
        DesktopCommand::Close(key) => managed(wire, key)?.close(),
        DesktopCommand::Minimise(key) => managed(wire, key)?.set_minimized(),
        DesktopCommand::Unminimise(key) => managed(wire, key)?.unset_minimized(),
        DesktopCommand::Maximise(key) => managed(wire, key)?.set_maximized(),
        DesktopCommand::Unmaximise(key) => managed(wire, key)?.unset_maximized(),
        DesktopCommand::Fullscreen { toplevel, output } => {
            let handle = managed(wire, toplevel)?;
            let output = output.as_ref().map(|n| output_of(wire, n)).transpose()?;
            fullscreen(handle, output)?;
        }
        DesktopCommand::Unfullscreen(key) => {
            let handle = managed(wire, key)?;
            if handle.version() >= zwlr_foreign_toplevel_handle_v1::REQ_UNSET_FULLSCREEN_SINCE {
                handle.unset_fullscreen();
            }
        }
        DesktopCommand::SetMinimiseTarget {
            toplevel,
            surface: key,
            rect,
        } => set_rectangle(managed(wire, toplevel)?, surface(wire, key)?, rect),
        _ => {}
    }
    Ok(())
}

/// `set_fullscreen` exists from version 2; an older handle cannot do it.
fn fullscreen(
    handle: &ZwlrForeignToplevelHandleV1,
    output: Option<&WlOutput>,
) -> Result<(), CommandError> {
    if handle.version() < zwlr_foreign_toplevel_handle_v1::REQ_SET_FULLSCREEN_SINCE {
        return Err(CommandError::Unsupported {
            command: crate::command::CommandKind::Fullscreen,
            needs: crate::protocol::Protocol::WlrForeignToplevelManagement,
        });
    }
    handle.set_fullscreen(output);
    Ok(())
}

fn set_rectangle(
    handle: &ZwlrForeignToplevelHandleV1,
    surface: &wayland_client::protocol::wl_surface::WlSurface,
    rect: Rect,
) {
    let clamp = |v: u32| i32::try_from(v).unwrap_or(i32::MAX);
    handle.set_rectangle(surface, rect.x, rect.y, clamp(rect.w), clamp(rect.h));
}

fn workspace_handle(
    wire: &Wire,
    key: WorkspaceKey,
) -> Result<
    &wayland_protocols::ext::workspace::v1::client::ext_workspace_handle_v1::ExtWorkspaceHandleV1,
    CommandError,
> {
    wire.handles
        .workspaces
        .get(&key)
        .ok_or(CommandError::UnknownWorkspace(key))
}

fn group_handle(
    wire: &Wire,
    key: GroupKey,
) -> Result<
    &wayland_protocols::ext::workspace::v1::client::ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1,
    CommandError,
>{
    wire.handles
        .groups
        .get(&key)
        .ok_or(CommandError::UnknownGroup(key))
}

/// The workspace commands (`ext-workspace-v1`); the compositor stages them until `commit`.
fn workspace(wire: &Wire, command: DesktopCommand) -> Result<(), CommandError> {
    match command {
        DesktopCommand::ActivateWorkspace(key) => workspace_handle(wire, key)?.activate(),
        DesktopCommand::DeactivateWorkspace(key) => workspace_handle(wire, key)?.deactivate(),
        DesktopCommand::RemoveWorkspace(key) => workspace_handle(wire, key)?.remove(),
        DesktopCommand::AssignWorkspace { workspace, group } => {
            workspace_handle(wire, workspace)?.assign(group_handle(wire, group)?);
        }
        DesktopCommand::CreateWorkspace { group, name } => {
            group_handle(wire, group)?.create_workspace(name);
        }
        DesktopCommand::CommitWorkspaces => {
            if let Some(manager) = &wire.handles.workspace_manager {
                manager.commit();
            }
        }
        _ => {}
    }
    Ok(())
}
