//! What to capture.

use crate::names::OutputName;

/// Whether the pointer cursor is painted onto the frame (`options.paint_cursors`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorMode {
    /// Not painted.
    #[default]
    Hidden,
    /// Painted.
    Painted,
}

/// A capture of one output.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct CaptureRequest {
    /// The output.
    pub output: OutputName,
    /// The cursor.
    pub cursor: CursorMode,
}

impl CaptureRequest {
    /// A capture of `output` with the cursor hidden.
    pub fn new(output: OutputName) -> Self {
        Self {
            output,
            cursor: CursorMode::Hidden,
        }
    }

    /// This capture with `cursor`.
    pub fn with_cursor(mut self, cursor: CursorMode) -> Self {
        self.cursor = cursor;
        self
    }
}
