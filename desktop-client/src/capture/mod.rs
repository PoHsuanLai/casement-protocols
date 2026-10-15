//! Output capture: `ext-image-copy-capture-v1` over `ext-output-image-capture-source-v1`.
//!
//! One request, one frame, into shared memory. The wire layer creates the source from the
//! named output, negotiates the buffer, captures once and reports [`CaptureEvent`]s;
//! [`Captures`] folds them into a [`Frame`] a consumer takes with [`Captures::take_ready`].

mod event;
mod frame;
mod request;
mod session;

pub use event::{CaptureEvent, CaptureFailure};
pub use frame::{Frame, FrameError, ShmFormat};
pub use request::{CaptureRequest, CursorMode, SpaceShot};
pub(crate) use session::reduce_captures;
pub use session::{CapturePhase, CaptureSession, Captures};
