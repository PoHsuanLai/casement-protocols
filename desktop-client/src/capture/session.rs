//! One capture, folded from its events.

use std::collections::BTreeMap;
use std::time::Duration;

use super::event::{CaptureEvent, CaptureFailure};
use super::frame::{Frame, ShmFormat};
use crate::geometry::{Rect, Size};
use crate::keys::CaptureId;
use crate::output::Transform;

/// Where a capture is.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CapturePhase {
    /// Waiting for the session's constraints (buffer size, formats).
    Negotiating,
    /// A buffer is attached and the capture is requested.
    Capturing,
    /// The frame is here, not yet taken.
    Ready(Frame),
    /// It failed.
    Failed(CaptureFailure),
}

/// One capture.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct CaptureSession {
    /// Its id.
    pub id: CaptureId,
    /// Where it is.
    pub phase: CapturePhase,
    /// The buffer size the session asked for, once known.
    pub size: Option<Size>,
    /// The shared-memory formats it accepts.
    pub formats: Vec<ShmFormat>,
    /// The frame's transform, once known.
    pub transform: Transform,
    /// The damage reported.
    pub damage: Vec<Rect>,
    /// When the frame was presented, if the compositor said.
    pub presented: Option<Duration>,
}

impl CaptureSession {
    /// A capture that has just been requested.
    pub fn new(id: CaptureId) -> Self {
        Self {
            id,
            phase: CapturePhase::Negotiating,
            size: None,
            formats: Vec::new(),
            transform: Transform::Normal,
            damage: Vec::new(),
            presented: None,
        }
    }
}

/// The captures in flight or waiting to be taken.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Captures(pub(crate) BTreeMap<CaptureId, CaptureSession>);

impl Captures {
    /// One capture.
    pub fn get(&self, id: CaptureId) -> Option<&CaptureSession> {
        self.0.get(&id)
    }

    /// Takes a finished capture out: its frame, or why it failed. `None` while it is still
    /// going or unknown.
    pub fn take_ready(mut self, id: CaptureId) -> (Self, Option<Result<Frame, CaptureFailure>>) {
        let Some(session) = self.0.remove(&id) else {
            return (self, None);
        };
        match session.phase {
            CapturePhase::Ready(frame) => (self, Some(Ok(frame))),
            CapturePhase::Failed(reason) => (self, Some(Err(reason))),
            CapturePhase::Negotiating | CapturePhase::Capturing => {
                self.0.insert(id, session);
                (self, None)
            }
        }
    }
}

/// `captures` after `event` for `id`; an event for an unknown id starts a session. A session
/// that is ready or failed ignores every later event.
pub(crate) fn reduce_captures(
    mut captures: Captures,
    id: CaptureId,
    event: CaptureEvent,
) -> Captures {
    let session = captures
        .0
        .entry(id)
        .or_insert_with(|| CaptureSession::new(id));
    let stage = match &session.phase {
        CapturePhase::Negotiating => Stage::Negotiating,
        CapturePhase::Capturing => Stage::Capturing,
        CapturePhase::Ready(_) | CapturePhase::Failed(_) => Stage::Finished,
    };
    match (stage, event) {
        (Stage::Finished, _) => {}
        (_, CaptureEvent::Failed(reason)) => session.phase = CapturePhase::Failed(reason),
        (Stage::Negotiating, CaptureEvent::BufferSize(size)) => session.size = Some(size),
        (Stage::Negotiating, CaptureEvent::Format(format)) => {
            if !session.formats.contains(&format) {
                session.formats.push(format);
            }
        }
        (Stage::Negotiating, CaptureEvent::ConstraintsDone) => {
            session.phase = constraints_phase(session);
        }
        (_, CaptureEvent::Transform(transform)) => session.transform = transform,
        (_, CaptureEvent::Damage(rect)) => session.damage.push(rect),
        (_, CaptureEvent::PresentationTime(time)) => session.presented = Some(time),
        (Stage::Capturing, CaptureEvent::Ready { stride, pixels }) => {
            session.phase = ready_phase(session, stride, pixels);
        }
        (Stage::Negotiating, CaptureEvent::Ready { .. }) => {
            session.phase = CapturePhase::Failed(CaptureFailure::BufferConstraints);
        }
        _ => {}
    }
    captures
}

/// The coarse position of a session, so the match does not borrow its phase.
#[derive(Debug, Clone, Copy)]
enum Stage {
    Negotiating,
    Capturing,
    Finished,
}

/// Where a session goes when its constraints are complete.
fn constraints_phase(session: &CaptureSession) -> CapturePhase {
    match (session.size, ShmFormat::pick(&session.formats)) {
        (Some(size), Some(_)) if size.w > 0 && size.h > 0 => CapturePhase::Capturing,
        (_, None) => CapturePhase::Failed(CaptureFailure::NoShmFormat),
        _ => CapturePhase::Failed(CaptureFailure::BufferConstraints),
    }
}

/// The phase after the buffer is filled.
fn ready_phase(session: &CaptureSession, stride: u32, pixels: Vec<u8>) -> CapturePhase {
    match (session.size, ShmFormat::pick(&session.formats)) {
        (Some(size), Some(format)) => {
            let mut frame = Frame::new(size, stride, format, pixels);
            frame.transform = session.transform;
            frame.damage = session.damage.clone();
            frame.presented = session.presented;
            CapturePhase::Ready(frame)
        }
        _ => CapturePhase::Failed(CaptureFailure::BufferConstraints),
    }
}
