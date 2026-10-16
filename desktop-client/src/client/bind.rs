//! Binding the globals.

use wayland_client::QueueHandle;
use wayland_client::globals::GlobalList;
use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::protocol::wl_shm::WlShm;

use super::error::WireError;
use super::wire::Wire;
use crate::backend::{DesktopBackend, backend_for};
use crate::config::ShellBinding;
use crate::protocol::{Protocol, ProtocolSet};

/// The `wl_output` version that carries `name`: the floor.
pub(super) const OUTPUT_VERSION: u32 = 4;

/// The offer in `globals`, as the protocols this crate uses.
pub(super) fn offered(globals: &GlobalList) -> ProtocolSet {
    globals
        .contents()
        .with_list(|list| ProtocolSet::from_interfaces(list.iter().map(|g| g.interface.as_str())))
}

/// Binds each offered protocol at the version this crate implements (`quire_desktop_v1` only on
/// the Quire arm, `quire_shell_v1` only when the caller holds the shell role), and the first seat, the
/// shm and every `wl_output` (at version 4 for `name`).
///
/// An output below version 4 is refused with [`WireError::OutputTooOld`]: without the name a
/// toplevel, workspace or head cannot be matched to its output.
pub(super) fn bind_all(
    wire: &mut Wire,
    globals: &GlobalList,
    qh: &QueueHandle<Wire>,
) -> Result<(), WireError> {
    bind_outputs(globals, qh)?;
    wire.handles.seat = globals.bind::<WlSeat, Wire, ()>(qh, 1..=1, ()).ok();
    wire.handles.shm = globals.bind::<WlShm, Wire, ()>(qh, 1..=1, ()).ok();
    let offered = wire.state.protocols.clone();
    if offered.contains(Protocol::ExtForeignToplevelList) {
        wire.handles.toplevel_list = Some(globals.bind(qh, 1..=1, ())?);
    }
    // On the Quire arm the toplevels come from the ext list and `quire_desktop_v1`; the wlr
    // manager is left unbound so one window is not reported twice.
    let backend = backend_for(&offered);
    if offered.contains(Protocol::WlrForeignToplevelManagement)
        && backend != Some(DesktopBackend::Quire)
    {
        wire.handles.toplevel_manager = Some(globals.bind(qh, 1..=3, ())?);
    }
    if backend == Some(DesktopBackend::Quire) {
        wire.handles.quire_manager = Some(globals.bind(qh, 1..=2, ())?);
    }
    if offered.contains(Protocol::QuireShell) && wire.config.shell == ShellBinding::Bind {
        wire.handles.quire_shell = Some(globals.bind(qh, 1..=6, ())?);
    }
    if offered.contains(Protocol::ExtWorkspace) {
        wire.handles.workspace_manager = Some(globals.bind(qh, 1..=1, ())?);
    }
    if offered.contains(Protocol::WlrOutputManagement) {
        wire.handles.output_manager = Some(globals.bind(qh, 1..=4, ())?);
    }
    if offered.contains(Protocol::ExtImageCopyCapture) {
        wire.handles.copy_capture = Some(globals.bind(qh, 1..=1, ())?);
    }
    if offered.contains(Protocol::ExtOutputImageCaptureSource) {
        wire.handles.output_source = Some(globals.bind(qh, 1..=1, ())?);
    }
    Ok(())
}

fn bind_outputs(globals: &GlobalList, qh: &QueueHandle<Wire>) -> Result<(), WireError> {
    let outputs: Vec<(u32, u32)> = globals.contents().with_list(|list| {
        list.iter()
            .filter(|g| g.interface == "wl_output")
            .map(|g| (g.name, g.version))
            .collect()
    });
    for (name, version) in outputs {
        if version < OUTPUT_VERSION {
            return Err(WireError::OutputTooOld {
                global: name,
                version,
            });
        }
        let _: WlOutput = globals.registry().bind(name, OUTPUT_VERSION, qh, ());
    }
    Ok(())
}
