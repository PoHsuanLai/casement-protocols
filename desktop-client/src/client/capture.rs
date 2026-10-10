//! Capture dispatch: session and frame (`ext-image-copy-capture-v1`).

use wayland_protocols::ext::image_copy_capture::v1::client::{
    ext_image_copy_capture_frame_v1, ext_image_copy_capture_session_v1,
};

use super::error::WireError;
use super::wire::Wire;
use crate::capture::CaptureRequest;
use crate::keys::CaptureId;

// The fill lane adds the `Dispatch` impls (session and frame, user data `CaptureId`). Flow:
// `start` makes the source from the named `wl_output`, creates the session and returns the id;
// on the session's `done` it makes a shm buffer of the announced size and format (`shm.rs`),
// creates the frame, attaches, damages the whole buffer and captures; on `ready` it reads the
// pixels back and emits `CaptureEvent::Ready`; on `failed` or `stopped` it emits `Failed`.

/// Starts a capture of the output `request` names.
pub(super) fn start(wire: &mut Wire, request: &CaptureRequest) -> Result<CaptureId, WireError> {
    let _ = (wire, request);
    todo!("fill")
}

/// The session's event.
pub(super) fn on_session(
    wire: &mut Wire,
    id: CaptureId,
    event: ext_image_copy_capture_session_v1::Event,
) {
    let _ = (wire, id, event);
    todo!("fill")
}

/// The frame's event.
pub(super) fn on_frame(
    wire: &mut Wire,
    id: CaptureId,
    event: ext_image_copy_capture_frame_v1::Event,
) {
    let _ = (wire, id, event);
    todo!("fill")
}
