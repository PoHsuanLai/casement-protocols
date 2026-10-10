//! The proxies, by key.

use std::collections::BTreeMap;

use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::protocol::wl_shm::WlShm;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_protocols::ext::foreign_toplevel_list::v1::client::ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1;
use wayland_protocols::ext::foreign_toplevel_list::v1::client::ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1;
use wayland_protocols::ext::image_capture_source::v1::client::ext_output_image_capture_source_manager_v1::ExtOutputImageCaptureSourceManagerV1;
use wayland_protocols::ext::image_copy_capture::v1::client::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1;
use wayland_protocols::ext::workspace::v1::client::ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1;
use wayland_protocols::ext::workspace::v1::client::ext_workspace_handle_v1::ExtWorkspaceHandleV1;
use wayland_protocols::ext::workspace::v1::client::ext_workspace_manager_v1::ExtWorkspaceManagerV1;
use wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1;
use wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_head_v1::ZwlrOutputHeadV1;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_manager_v1::ZwlrOutputManagerV1;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_mode_v1::ZwlrOutputModeV1;

use casement_protocols::client::quire_desktop_v1::{
    quire_desktop_app_v1::QuireDesktopAppV1, quire_desktop_manager_v1::QuireDesktopManagerV1,
    quire_desktop_space_v1::QuireDesktopSpaceV1, quire_desktop_window_v1::QuireDesktopWindowV1,
};
use casement_protocols::client::quire_shell_v1::quire_shell_v1::QuireShellV1;

use super::capture::Live;
use crate::keys::{
    AppKey, CaptureId, GroupKey, HeadKey, ModeKey, SpaceKey, SurfaceKey, ToplevelKey, WorkspaceKey,
};
use crate::names::OutputName;
use crate::quire::WindowObject;

/// The bound globals (each present only when offered) and the objects they created.
#[derive(Debug, Default)]
pub(super) struct Handles {
    pub(super) toplevel_list: Option<ExtForeignToplevelListV1>,
    pub(super) toplevel_manager: Option<ZwlrForeignToplevelManagerV1>,
    pub(super) workspace_manager: Option<ExtWorkspaceManagerV1>,
    pub(super) output_manager: Option<ZwlrOutputManagerV1>,
    pub(super) copy_capture: Option<ExtImageCopyCaptureManagerV1>,
    pub(super) output_source: Option<ExtOutputImageCaptureSourceManagerV1>,
    pub(super) quire_manager: Option<QuireDesktopManagerV1>,
    /// Bound only when the caller holds the shell role.
    pub(super) quire_shell: Option<QuireShellV1>,
    pub(super) shm: Option<WlShm>,
    /// The seat activation acts on: the first one announced.
    pub(super) seat: Option<WlSeat>,
    /// `wl_output`s by the name they announced, for capture sources and fullscreen.
    pub(super) outputs: BTreeMap<OutputName, WlOutput>,
    /// The name each `wl_output` announced, by its protocol id.
    pub(super) output_names: BTreeMap<u32, OutputName>,
    pub(super) listed: BTreeMap<ToplevelKey, ExtForeignToplevelHandleV1>,
    pub(super) managed: BTreeMap<ToplevelKey, ZwlrForeignToplevelHandleV1>,
    pub(super) workspaces: BTreeMap<WorkspaceKey, ExtWorkspaceHandleV1>,
    pub(super) groups: BTreeMap<GroupKey, ExtWorkspaceGroupHandleV1>,
    pub(super) heads: BTreeMap<HeadKey, ZwlrOutputHeadV1>,
    pub(super) modes: BTreeMap<ModeKey, ZwlrOutputModeV1>,
    pub(super) apps: BTreeMap<AppKey, QuireDesktopAppV1>,
    pub(super) spaces: BTreeMap<SpaceKey, QuireDesktopSpaceV1>,
    pub(super) windows: BTreeMap<WindowObject, QuireDesktopWindowV1>,
    pub(super) surfaces: BTreeMap<SurfaceKey, WlSurface>,
    /// Captures in flight.
    pub(super) captures: BTreeMap<CaptureId, Live>,
}
