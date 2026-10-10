//! What a capture session and its frame report.

use std::time::Duration;

use super::frame::ShmFormat;
use crate::geometry::{Rect, Size};
use crate::output::Transform;

/// Why a capture failed (`ext_image_copy_capture_frame_v1.failure_reason` and the session's
/// `stopped`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum CaptureFailure {
    /// The compositor gave no reason.
    #[error("the compositor failed the capture")]
    Unknown,
    /// The buffer did not match what the session asked for.
    #[error("the buffer did not meet the session's constraints")]
    BufferConstraints,
    /// The session stopped (the output went away, or the compositor refused).
    #[error("the capture session stopped")]
    Stopped,
    /// The compositor offered no shared-memory format, only dma-buf.
    #[error("the compositor offers no shared-memory format")]
    NoShmFormat,
}

/// A capture event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CaptureEvent {
    /// The session wants a buffer of this size (`buffer_size`).
    BufferSize(Size),
    /// A shared-memory format it accepts (`shm_format`, repeated).
    Format(ShmFormat),
    /// The constraints are complete (`done`).
    ConstraintsDone,
    /// The frame's transform.
    Transform(Transform),
    /// A damaged region of the buffer.
    Damage(Rect),
    /// When the frame was presented, as time since the compositor's clock origin.
    PresentationTime(Duration),
    /// The buffer is filled; `pixels` is its content, `stride` bytes per row (`ready`).
    Ready {
        /// Bytes per row.
        stride: u32,
        /// The content, `stride * height` bytes.
        pixels: Vec<u8>,
    },
    /// The capture failed.
    Failed(CaptureFailure),
}
