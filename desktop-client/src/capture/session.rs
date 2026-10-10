//! One capture, folded from its events.

use std::collections::BTreeMap;

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
    pub fn take_ready(self, id: CaptureId) -> (Self, Option<Result<Frame, CaptureFailure>>) {
        let _ = id;
        todo!("fill: remove the entry when Ready or Failed")
    }
}

/// `captures` after `event` for `id`; an event for an unknown id starts a session.
pub(crate) fn reduce_captures(captures: Captures, id: CaptureId, event: CaptureEvent) -> Captures {
    let _ = (captures, id, event);
    todo!("fill: Negotiating -> Capturing at ConstraintsDone; Ready builds the Frame")
}
