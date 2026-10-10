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

    /// Whether [`Frame::to_rgba8`] reads this format.
    pub const fn is_supported(self) -> bool {
        matches!(
            self,
            Self::ARGB8888 | Self::XRGB8888 | Self::ABGR8888 | Self::XBGR8888
        )
    }

    /// The first of `offered` that [`Frame::to_rgba8`] reads. The wire layer and the capture
    /// reducer both choose with this, so they agree.
    pub fn pick(offered: &[ShmFormat]) -> Option<ShmFormat> {
        offered.iter().copied().find(|f| f.is_supported())
    }

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
    /// A frame of `size` whose rows are `stride` bytes apart; no damage, no presentation time,
    /// the upright transform.
    pub fn new(size: Size, stride: u32, format: ShmFormat, pixels: Vec<u8>) -> Self {
        Self {
            size,
            stride,
            format,
            transform: Transform::Normal,
            damage: Vec::new(),
            presented: None,
            pixels,
        }
    }

    /// The frame as tightly packed RGBA8, rows top to bottom, ready for a PNG encoder.
    /// `xrgb`/`xbgr` get an opaque alpha. The transform is not applied.
    ///
    /// ```
    /// use desktop_client::capture::ShmFormat;
    /// use desktop_client::{Frame, Size};
    ///
    /// // One xrgb8888 pixel: B, G, R, unused.
    /// let frame = Frame::new(Size::new(1, 1), 4, ShmFormat::XRGB8888, vec![3, 2, 1, 0]);
    /// assert_eq!(frame.to_rgba8().unwrap(), vec![1, 2, 3, 255]);
    /// ```
    pub fn to_rgba8(&self) -> Result<Vec<u8>, FrameError> {
        let swizzle: fn([u8; 4]) -> [u8; 4] = match self.format {
            ShmFormat::ARGB8888 => |p| [p[2], p[1], p[0], p[3]],
            ShmFormat::XRGB8888 => |p| [p[2], p[1], p[0], 255],
            ShmFormat::ABGR8888 => |p| p,
            ShmFormat::XBGR8888 => |p| [p[0], p[1], p[2], 255],
            other => return Err(FrameError::UnsupportedFormat(other)),
        };
        let width = self.size.w as usize;
        let height = self.size.h as usize;
        let stride = self.stride as usize;
        let row = width.saturating_mul(4);
        if stride < row {
            return Err(FrameError::Stride {
                stride: self.stride,
                row,
            });
        }
        let need = stride.saturating_mul(height);
        if self.pixels.len() < need {
            return Err(FrameError::Short {
                have: self.pixels.len(),
                need,
            });
        }
        let mut out = Vec::with_capacity(row.saturating_mul(height));
        for line in self.pixels.chunks_exact(stride.max(1)).take(height) {
            for p in line[..row].chunks_exact(4) {
                out.extend_from_slice(&swizzle([p[0], p[1], p[2], p[3]]));
            }
        }
        Ok(out)
    }
}

/// A frame that cannot be converted.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum FrameError {
    /// This crate cannot read that format.
    #[error("unsupported shm format {0:?}")]
    UnsupportedFormat(ShmFormat),
    /// A row of pixels does not fit in the stride.
    #[error("stride {stride} is shorter than a {row}-byte row")]
    Stride {
        /// The stride given.
        stride: u32,
        /// The bytes a row of pixels needs.
        row: usize,
    },
    /// The pixel buffer is shorter than `stride * height`.
    #[error("{have} bytes of pixels, {need} needed")]
    Short {
        /// What arrived.
        have: usize,
        /// What the size and stride need.
        need: usize,
    },
}
