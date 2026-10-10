//! A captured frame.

use std::time::Duration;

use crate::geometry::{Rect, Size};
use crate::output::Transform;

/// A `wl_shm` pixel format by its wire code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShmFormat(u32);

impl ShmFormat {
    /// `argb8888`: B, G, R, A bytes on a little-endian machine.
    pub const ARGB8888: ShmFormat = ShmFormat(0);
    /// `xrgb8888`: B, G, R, unused.
    pub const XRGB8888: ShmFormat = ShmFormat(1);
    /// `abgr8888`: R, G, B, A bytes.
    pub const ABGR8888: ShmFormat = ShmFormat(0x3432_4241);
    /// `xbgr8888`: R, G, B, unused.
    pub const XBGR8888: ShmFormat = ShmFormat(0x3432_4258);

    /// The format with this `wl_shm.format` code.
    pub const fn from_code(code: u32) -> Self {
        Self(code)
    }

    /// The `wl_shm.format` code.
    pub const fn code(self) -> u32 {
        self.0
    }
}

/// One captured frame of an output.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Frame {
    /// Pixels.
    pub size: Size,
    /// Bytes per row.
    pub stride: u32,
    /// How the pixels are laid out.
    pub format: ShmFormat,
    /// How the compositor says to rotate or flip the frame to show it upright.
    pub transform: Transform,
    /// The regions that changed since the last frame of the session.
    pub damage: Vec<Rect>,
    /// When it was presented, if the compositor said.
    pub presented: Option<Duration>,
    /// The pixels, `stride * size.h` bytes.
    pub pixels: Vec<u8>,
}

impl Frame {
    /// The frame as tightly packed RGBA8, rows top to bottom, ready for a PNG encoder.
    /// `xrgb`/`xbgr` get an opaque alpha. The transform is not applied.
    pub fn to_rgba8(&self) -> Result<Vec<u8>, FrameError> {
        todo!(
            "fill: check the length against stride and height, swizzle per format, drop the row padding"
        )
    }
}

/// A frame that cannot be converted.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum FrameError {
    /// This crate cannot read that format.
    #[error("unsupported shm format {0:?}")]
    UnsupportedFormat(ShmFormat),
    /// The pixel buffer is shorter than `stride * height`.
    #[error("{have} bytes of pixels, {need} needed")]
    Short {
        /// What arrived.
        have: usize,
        /// What the size and stride need.
        need: usize,
    },
}
