//! What to capture.

use crate::geometry::Size;
use crate::names::{OutputName, SpaceId};

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
    /// The Space of `output` to capture instead of the whole output, and the size of the
    /// picture (quire_shell_v1 version 2); `None` for the output as it is shown.
    pub space: Option<SpaceShot>,
}

/// A picture of one Space: which, and how big.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SpaceShot {
    /// The Space.
    pub space: SpaceId,
    /// The size of the picture in pixels (the compositor keeps each side between 1 and 1024).
    pub size: Size,
}

impl CaptureRequest {
    /// A capture of `output` with the cursor hidden.
    pub fn new(output: OutputName) -> Self {
        Self {
            output,
            cursor: CursorMode::Hidden,
            space: None,
        }
    }

    /// A picture of `space` of `output`, `size` pixels, with the cursor hidden. The windows are
    /// drawn without frames over a transparent background; ask again when the Space changed.
    pub fn of_space(output: OutputName, space: SpaceId, size: Size) -> Self {
        Self {
            space: Some(SpaceShot { space, size }),
            ..Self::new(output)
        }
    }

    /// This capture with `cursor`.
    pub fn with_cursor(mut self, cursor: CursorMode) -> Self {
        self.cursor = cursor;
        self
    }
}
