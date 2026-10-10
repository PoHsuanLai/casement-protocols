//! Objects whose events this client does not use, and the registry.

use wayland_client::globals::GlobalListContents;
use wayland_client::protocol::wl_buffer::{self, WlBuffer};
use wayland_client::protocol::wl_output::{self, WlOutput};
use wayland_client::protocol::wl_registry::{self, WlRegistry};
use wayland_client::protocol::wl_seat::{self, WlSeat};
use wayland_client::protocol::wl_shm::{self, WlShm};
use wayland_client::protocol::wl_shm_pool::{self, WlShmPool};
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::ext::image_capture_source::v1::client::{
    ext_image_capture_source_v1::{self, ExtImageCaptureSourceV1},
    ext_output_image_capture_source_manager_v1::{self, ExtOutputImageCaptureSourceManagerV1},
};
use wayland_protocols::ext::image_copy_capture::v1::client::ext_image_copy_capture_manager_v1::{
    self, ExtImageCopyCaptureManagerV1,
};

use super::bind::OUTPUT_VERSION;
use super::wire::Wire;
use crate::names::OutputName;
use wayland_client::Proxy;

/// Implements a `Dispatch` that ignores every event.
macro_rules! ignore {
    ($($proxy:ty => $module:ident),* $(,)?) => {$(
        impl Dispatch<$proxy, ()> for Wire {
            fn event(
                _state: &mut Self,
                _proxy: &$proxy,
                _event: $module::Event,
                _data: &(),
                _conn: &Connection,
                _qh: &QueueHandle<Self>,
            ) {
            }
        }
    )*};
}

ignore! {
    WlSeat => wl_seat,
    WlShm => wl_shm,
    WlShmPool => wl_shm_pool,
    WlBuffer => wl_buffer,
    ExtImageCaptureSourceV1 => ext_image_capture_source_v1,
    ExtOutputImageCaptureSourceManagerV1 => ext_output_image_capture_source_manager_v1,
    ExtImageCopyCaptureManagerV1 => ext_image_copy_capture_manager_v1,
}

/// A `wl_output` tells its name here; that is how a `WlOutput` becomes an `OutputName`.
impl Dispatch<WlOutput, ()> for Wire {
    fn event(
        state: &mut Self,
        proxy: &WlOutput,
        event: wl_output::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        if let wl_output::Event::Name { name } = event {
            let name = OutputName::new(name);
            state
                .handles
                .output_names
                .insert(proxy.id().protocol_id(), name.clone());
            state.handles.outputs.insert(name, proxy.clone());
        }
    }
}

/// An output that appears after the first listing is bound as the first ones were; one that
/// is too old is skipped (the start-up listing refuses it, see `bind_all`).
impl Dispatch<WlRegistry, GlobalListContents> for Wire {
    fn event(
        _state: &mut Self,
        registry: &WlRegistry,
        event: wl_registry::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
            && interface == "wl_output"
            && version >= OUTPUT_VERSION
        {
            // The output announces its name; the handler above records it.
            let _: WlOutput = registry.bind(name, OUTPUT_VERSION, qh, ());
        }
    }
}
