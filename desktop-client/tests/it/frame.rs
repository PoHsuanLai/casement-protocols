//! `Frame::to_rgba8` and format picking.

use desktop_client::capture::{FrameError, ShmFormat};
use desktop_client::{Frame, Size};

fn frame(format: ShmFormat, size: (u32, u32), stride: u32, pixels: Vec<u8>) -> Frame {
    Frame::new(Size::new(size.0, size.1), stride, format, pixels)
}

#[test]
fn each_supported_format_swizzles_to_rgba() {
    // One pixel whose bytes in memory are 1, 2, 3, 4.
    let cases = [
        (ShmFormat::ARGB8888, vec![3, 2, 1, 4]),
        (ShmFormat::XRGB8888, vec![3, 2, 1, 255]),
        (ShmFormat::ABGR8888, vec![1, 2, 3, 4]),
        (ShmFormat::XBGR8888, vec![1, 2, 3, 255]),
    ];
    for (format, expected) in cases {
        let rgba = frame(format, (1, 1), 4, vec![1, 2, 3, 4])
            .to_rgba8()
            .unwrap();
        assert_eq!(rgba, expected, "{format:?}");
    }
}

#[test]
fn row_padding_is_dropped() {
    // Two rows of one pixel, stride 8: four bytes of padding after each pixel.
    let pixels = vec![
        10, 20, 30, 40, 0xee, 0xee, 0xee, 0xee, //
        11, 21, 31, 41, 0xee, 0xee, 0xee, 0xee,
    ];
    let rgba = frame(ShmFormat::ABGR8888, (1, 2), 8, pixels)
        .to_rgba8()
        .unwrap();
    assert_eq!(rgba, vec![10, 20, 30, 40, 11, 21, 31, 41]);
}

#[test]
fn a_buffer_shorter_than_stride_times_height_is_short() {
    // Exactly stride * height bytes are required: the last row is not exempt from its padding.
    let short = frame(ShmFormat::ABGR8888, (1, 2), 8, vec![0; 12]);
    assert_eq!(
        short.to_rgba8(),
        Err(FrameError::Short { have: 12, need: 16 })
    );
}

#[test]
fn extra_bytes_are_ignored() {
    let rgba = frame(ShmFormat::ABGR8888, (1, 1), 4, vec![1, 2, 3, 4, 9, 9])
        .to_rgba8()
        .unwrap();
    assert_eq!(rgba, vec![1, 2, 3, 4]);
}

#[test]
fn unsupported_formats_are_typed_errors() {
    let nv12 = ShmFormat::from_code(0x3231_564e);
    assert_eq!(
        frame(nv12, (1, 1), 4, vec![0; 4]).to_rgba8(),
        Err(FrameError::UnsupportedFormat(nv12))
    );
}

#[test]
fn a_stride_shorter_than_a_row_is_a_typed_error() {
    assert_eq!(
        frame(ShmFormat::ARGB8888, (2, 1), 4, vec![0; 8]).to_rgba8(),
        Err(FrameError::Stride { stride: 4, row: 8 })
    );
}

#[test]
fn an_empty_frame_is_empty() {
    assert_eq!(
        frame(ShmFormat::ARGB8888, (0, 0), 0, vec![]).to_rgba8(),
        Ok(vec![])
    );
}

#[test]
fn pick_takes_the_first_readable_format() {
    let nv12 = ShmFormat::from_code(0x3231_564e);
    assert_eq!(
        ShmFormat::pick(&[nv12, ShmFormat::XBGR8888, ShmFormat::ARGB8888]),
        Some(ShmFormat::XBGR8888)
    );
    assert_eq!(ShmFormat::pick(&[nv12]), None);
    assert_eq!(ShmFormat::pick(&[]), None);
}

#[test]
fn format_codes_are_the_wl_shm_codes() {
    assert_eq!(ShmFormat::ARGB8888.code(), 0);
    assert_eq!(ShmFormat::XRGB8888.code(), 1);
    assert_eq!(ShmFormat::ABGR8888.code(), u32::from_le_bytes(*b"AB24"));
    assert_eq!(ShmFormat::XBGR8888.code(), u32::from_le_bytes(*b"XB24"));
    assert!(ShmFormat::ARGB8888.is_supported());
    assert!(!ShmFormat::from_code(7).is_supported());
}
