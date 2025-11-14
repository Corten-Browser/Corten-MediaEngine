//! Unit tests for format enums

use cortenbrowser_shared_types::{AudioFormat, PixelFormat};

#[test]
fn test_pixel_format_variants() {
    let formats = vec![
        PixelFormat::YUV420,
        PixelFormat::YUV422,
        PixelFormat::YUV444,
        PixelFormat::RGB24,
        PixelFormat::RGBA32,
        PixelFormat::NV12,
    ];

    for format in formats {
        let debug = format!("{:?}", format);
        assert!(!debug.is_empty());
    }
}

#[test]
fn test_audio_format_variants() {
    let formats = vec![
        AudioFormat::F32LE,
        AudioFormat::S16LE,
        AudioFormat::S24LE,
        AudioFormat::S32LE,
    ];

    for format in formats {
        let debug = format!("{:?}", format);
        assert!(!debug.is_empty());
    }
}

#[test]
fn test_pixel_format_clone() {
    let fmt1 = PixelFormat::YUV420;
    let fmt2 = fmt1.clone();
    assert!(matches!(fmt2, PixelFormat::YUV420));
}

#[test]
fn test_audio_format_clone() {
    let fmt1 = AudioFormat::F32LE;
    let fmt2 = fmt1.clone();
    assert!(matches!(fmt2, AudioFormat::F32LE));
}

#[test]
fn test_pixel_format_eq() {
    assert_eq!(PixelFormat::YUV420, PixelFormat::YUV420);
    assert_ne!(PixelFormat::YUV420, PixelFormat::YUV422);
}

#[test]
fn test_audio_format_eq() {
    assert_eq!(AudioFormat::F32LE, AudioFormat::F32LE);
    assert_ne!(AudioFormat::F32LE, AudioFormat::S16LE);
}
