//! The capture buffer: an anonymous file, no mmap, safe code only.
//!
//! The buffer is a memfd (`rustix::fs::memfd_create`) sized with `set_len`, shared with the
//! compositor as a `wl_shm_pool`; after `ready` the pixels are read back with
//! `FileExt::read_exact_at`.

use std::fs::File;
use std::io;
use std::os::unix::fs::FileExt;

use rustix::fs::{MemfdFlags, memfd_create};

use super::error::WireError;
use crate::geometry::Size;

/// Bytes per pixel of every format `Frame::to_rgba8` reads.
const BYTES_PER_PIXEL: u32 = 4;

/// A buffer file for `size` pixels at `stride` bytes per row.
#[derive(Debug)]
pub(super) struct ShmFile {
    pub(super) file: File,
    pub(super) stride: u32,
    pub(super) size: Size,
}

impl ShmFile {
    /// Creates the file at its final length.
    pub(super) fn create(size: Size) -> Result<Self, WireError> {
        let stride = size.w.checked_mul(BYTES_PER_PIXEL).ok_or_else(too_big)?;
        let length = u64::from(stride) * u64::from(size.h);
        if i32::try_from(length).is_err() {
            return Err(too_big().into());
        }
        let fd =
            memfd_create("desktop-client-capture", MemfdFlags::CLOEXEC).map_err(io::Error::from)?;
        let file = File::from(fd);
        file.set_len(length)?;
        Ok(Self { file, stride, size })
    }

    /// The buffer's length in bytes (fits an `i32`, checked at creation).
    pub(super) fn length(&self) -> i32 {
        i32::try_from(u64::from(self.stride) * u64::from(self.size.h)).unwrap_or(i32::MAX)
    }

    /// Reads the whole buffer back.
    pub(super) fn read_pixels(&self) -> Result<Vec<u8>, WireError> {
        let length = usize::try_from(u64::from(self.stride) * u64::from(self.size.h))
            .map_err(|_| too_big())?;
        let mut pixels = vec![0u8; length];
        self.file.read_exact_at(&mut pixels, 0)?;
        Ok(pixels)
    }
}

fn too_big() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        "the capture buffer does not fit a wl_shm pool",
    )
}
