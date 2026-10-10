//! The capture buffer: an anonymous file, no mmap, safe code only.
//!
//! The buffer is a memfd (`rustix::fs::memfd_create`) sized with `set_len`, shared with the
//! compositor as a `wl_shm_pool`; after `ready` the pixels are read back with
//! `FileExt::read_exact_at`.

use std::fs::File;

use super::error::WireError;
use crate::capture::ShmFormat;
use crate::geometry::Size;

/// A buffer file for `size` pixels at `stride` bytes per row.
#[derive(Debug)]
pub(super) struct ShmFile {
    pub(super) file: File,
    pub(super) stride: u32,
    pub(super) format: ShmFormat,
    pub(super) size: Size,
}

impl ShmFile {
    /// Creates the file at its final length.
    pub(super) fn create(size: Size, format: ShmFormat) -> Result<Self, WireError> {
        let _ = (size, format);
        todo!("fill: memfd_create, set_len(stride * h), stride = w * 4")
    }

    /// Reads the whole buffer back.
    pub(super) fn read_pixels(&self) -> Result<Vec<u8>, WireError> {
        todo!("fill: read_exact_at(0)")
    }
}
