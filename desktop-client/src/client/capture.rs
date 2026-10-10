//! Capture dispatch: session and frame (`ext-image-copy-capture-v1`).
//!
//! Flow: `start` makes the source from the named `wl_output` and the session, and returns the
//! id. On the session's `done` the constraints are complete: if the reducer put the session in
//! `Capturing`, `begin_frame` makes a shm buffer of the announced size and format, creates the
//! frame, attaches it, damages all of it and captures. On `ready` the pixels are read back
//! and `CaptureEvent::Ready` is emitted; on `failed` or `stopped`, `CaptureEvent::Failed`.

use std::os::fd::AsFd;
use std::time::Duration;

use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_shm;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::ext::image_capture_source::v1::client::ext_image_capture_source_v1::ExtImageCaptureSourceV1;
use wayland_protocols::ext::image_copy_capture::v1::client::{
    ext_image_copy_capture_frame_v1::{self, ExtImageCopyCaptureFrameV1},
    ext_image_copy_capture_manager_v1::Options,
    ext_image_copy_capture_session_v1::{self, ExtImageCopyCaptureSessionV1},
};

use super::error::WireError;
use super::shm::ShmFile;
use super::wire::{Wire, raw};
use crate::capture::{
    CaptureEvent, CaptureFailure, CapturePhase, CaptureRequest, CursorMode, ShmFormat,
};
use crate::command::CommandKind;
use crate::error::CommandError;
use crate::event::DesktopEvent;
use crate::geometry::{Rect, Size};
use crate::keys::CaptureId;
use crate::output::Transform;
use crate::protocol::Protocol;

/// The protocol objects of one capture in flight.
#[derive(Debug)]
pub(super) struct Live {
    source: ExtImageCaptureSourceV1,
    session: ExtImageCopyCaptureSessionV1,
    frame: Option<ExtImageCopyCaptureFrameV1>,
    buffer: Option<WlBuffer>,
    shm: Option<ShmFile>,
}

impl Live {
    fn destroy(self) {
        if let Some(frame) = self.frame {
            frame.destroy();
        }
        if let Some(buffer) = self.buffer {
            buffer.destroy();
        }
        self.session.destroy();
        self.source.destroy();
    }
}

fn capture_event(wire: &mut Wire, id: CaptureId, event: CaptureEvent) {
    wire.emit(DesktopEvent::Capture { id, event });
}

/// Reports a failure and releases the capture's objects.
fn fail(wire: &mut Wire, id: CaptureId, reason: CaptureFailure) {
    capture_event(wire, id, CaptureEvent::Failed(reason));
    release(wire, id);
}

fn release(wire: &mut Wire, id: CaptureId) {
    if let Some(live) = wire.handles.captures.remove(&id) {
        live.destroy();
    }
}

/// Starts a capture of the output `request` names.
pub(super) fn start(wire: &mut Wire, request: &CaptureRequest) -> Result<CaptureId, WireError> {
    let id = CaptureId::new(wire.mint());
    let unsupported = |needs| CommandError::Unsupported {
        command: CommandKind::CaptureOutput,
        needs,
    };
    let output = wire
        .handles
        .outputs
        .get(&request.output)
        .ok_or_else(|| CommandError::UnknownOutput(request.output.clone()))?;
    let sources = wire
        .handles
        .output_source
        .as_ref()
        .ok_or_else(|| unsupported(Protocol::ExtOutputImageCaptureSource))?;
    let manager = wire
        .handles
        .copy_capture
        .as_ref()
        .ok_or_else(|| unsupported(Protocol::ExtImageCopyCapture))?;
    let options = match request.cursor {
        CursorMode::Hidden => Options::empty(),
        CursorMode::Painted => Options::PaintCursors,
    };
    let source = sources.create_source(output, &wire.qh, ());
    let session = manager.create_session(&source, options, &wire.qh, id);
    wire.handles.captures.insert(
        id,
        Live {
            source,
            session,
            frame: None,
            buffer: None,
            shm: None,
        },
    );
    Ok(id)
}

/// The session's event.
pub(super) fn on_session(
    wire: &mut Wire,
    id: CaptureId,
    event: ext_image_copy_capture_session_v1::Event,
) {
    use ext_image_copy_capture_session_v1::Event;
    match event {
        Event::BufferSize { width, height } => {
            capture_event(wire, id, CaptureEvent::BufferSize(Size::new(width, height)));
        }
        Event::ShmFormat { format } => {
            capture_event(
                wire,
                id,
                CaptureEvent::Format(ShmFormat::from_code(raw(format))),
            );
        }
        Event::Done => {
            capture_event(wire, id, CaptureEvent::ConstraintsDone);
            let phase = wire.state.captures.get(id).map(|s| s.phase.clone());
            match phase {
                Some(CapturePhase::Capturing) => begin_frame(wire, id),
                Some(CapturePhase::Failed(_)) => release(wire, id),
                _ => {}
            }
        }
        Event::Stopped => fail(wire, id, CaptureFailure::Stopped),
        _ => {}
    }
}

/// Makes the buffer and the frame and asks for the capture.
fn begin_frame(wire: &mut Wire, id: CaptureId) {
    if wire
        .handles
        .captures
        .get(&id)
        .is_some_and(|l| l.frame.is_some())
    {
        return;
    }
    let made = make_frame(wire, id);
    if made.is_none() {
        fail(wire, id, CaptureFailure::Unknown);
    }
}

fn make_frame(wire: &mut Wire, id: CaptureId) -> Option<()> {
    let session = wire.state.captures.get(id)?;
    let size = session.size?;
    let format = ShmFormat::pick(&session.formats)?;
    let shm_format = wl_shm::Format::try_from(format.code()).ok()?;
    let file = ShmFile::create(size).ok()?;
    let shm = wire.handles.shm.as_ref()?;
    let pool = shm.create_pool(file.file.as_fd(), file.length(), &wire.qh, ());
    let buffer = pool.create_buffer(
        0,
        i32::try_from(size.w).ok()?,
        i32::try_from(size.h).ok()?,
        i32::try_from(file.stride).ok()?,
        shm_format,
        &wire.qh,
        (),
    );
    pool.destroy();
    let live = wire.handles.captures.get_mut(&id)?;
    let frame = live.session.create_frame(&wire.qh, id);
    frame.attach_buffer(&buffer);
    frame.damage_buffer(
        0,
        0,
        i32::try_from(size.w).ok()?,
        i32::try_from(size.h).ok()?,
    );
    frame.capture();
    live.frame = Some(frame);
    live.buffer = Some(buffer);
    live.shm = Some(file);
    Some(())
}

/// The frame's event.
pub(super) fn on_frame(
    wire: &mut Wire,
    id: CaptureId,
    event: ext_image_copy_capture_frame_v1::Event,
) {
    use ext_image_copy_capture_frame_v1::Event;
    match event {
        Event::Transform { transform } => {
            if let Some(transform) = Transform::from_code(raw(transform)) {
                capture_event(wire, id, CaptureEvent::Transform(transform));
            }
        }
        Event::Damage {
            x,
            y,
            width,
            height,
        } => {
            let extent = |v: i32| u32::try_from(v).unwrap_or(0);
            capture_event(
                wire,
                id,
                CaptureEvent::Damage(Rect::new(x, y, extent(width), extent(height))),
            );
        }
        Event::PresentationTime {
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        } => {
            let seconds = (u64::from(tv_sec_hi) << 32) | u64::from(tv_sec_lo);
            capture_event(
                wire,
                id,
                CaptureEvent::PresentationTime(Duration::new(seconds, tv_nsec)),
            );
        }
        Event::Ready => ready(wire, id),
        Event::Failed { reason } => {
            let reason = match raw(reason) {
                1 => CaptureFailure::BufferConstraints,
                2 => CaptureFailure::Stopped,
                _ => CaptureFailure::Unknown,
            };
            fail(wire, id, reason);
        }
        _ => {}
    }
}

/// The buffer is filled: reads it back.
fn ready(wire: &mut Wire, id: CaptureId) {
    let read = wire
        .handles
        .captures
        .get(&id)
        .and_then(|live| live.shm.as_ref())
        .map(|shm| (shm.stride, shm.read_pixels()));
    match read {
        Some((stride, Ok(pixels))) => {
            capture_event(wire, id, CaptureEvent::Ready { stride, pixels });
            release(wire, id);
        }
        _ => fail(wire, id, CaptureFailure::Unknown),
    }
}

impl Dispatch<ExtImageCopyCaptureSessionV1, CaptureId> for Wire {
    fn event(
        state: &mut Self,
        _proxy: &ExtImageCopyCaptureSessionV1,
        event: ext_image_copy_capture_session_v1::Event,
        data: &CaptureId,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_session(state, *data, event);
    }
}

impl Dispatch<ExtImageCopyCaptureFrameV1, CaptureId> for Wire {
    fn event(
        state: &mut Self,
        _proxy: &ExtImageCopyCaptureFrameV1,
        event: ext_image_copy_capture_frame_v1::Event,
        data: &CaptureId,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        on_frame(state, *data, event);
    }
}
