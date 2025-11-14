//! Unit tests for VP9 decoder
//!
//! Following TDD: These tests are written FIRST (RED phase)

use cortenbrowser_shared_types::{MediaError, VideoDecoder, VideoPacket};
use cortenbrowser_video_decoders::VP9Decoder;

#[test]
fn test_vp9_decoder_creation() {
    //! Test that VP9 decoder can be created successfully

    let result = VP9Decoder::new();
    assert!(result.is_ok(), "VP9Decoder::new() should succeed");
}

#[test]
fn test_vp9_decoder_decode_valid_packet() {
    //! Test decoding a valid VP9 packet
    //!
    //! Given a valid VP9 packet
    //! When decode is called
    //! Then a VideoFrame should be returned

    let mut decoder = VP9Decoder::new().expect("Failed to create decoder");

    let packet = VideoPacket {
        data: create_test_vp9_frame(),
        pts: Some(0),
        dts: Some(0),
        is_keyframe: true,
    };

    let result = decoder.decode(&packet);
    assert!(result.is_ok(), "Decoding valid VP9 packet should succeed");

    let frame = result.unwrap();
    assert!(frame.width > 0, "Frame width should be positive");
    assert!(frame.height > 0, "Frame height should be positive");
}

#[test]
fn test_vp9_decoder_decode_invalid_packet() {
    //! Test decoding an invalid packet
    //!
    //! Given an invalid VP9 packet
    //! When decode is called
    //! Then a CodecError should be returned

    let mut decoder = VP9Decoder::new().expect("Failed to create decoder");

    let packet = VideoPacket {
        data: vec![0xFF, 0xFF, 0xFF],
        pts: Some(0),
        dts: Some(0),
        is_keyframe: false,
    };

    let result = decoder.decode(&packet);
    assert!(result.is_err(), "Decoding invalid VP9 packet should fail");
}

#[test]
fn test_vp9_decoder_flush() {
    //! Test flushing VP9 decoder

    let mut decoder = VP9Decoder::new().expect("Failed to create decoder");

    let result = decoder.flush();
    assert!(result.is_ok(), "VP9 flush should succeed");
}

// Helper function to create test VP9 frame data
fn create_test_vp9_frame() -> Vec<u8> {
    // Minimal VP9 keyframe header for testing
    // Real implementation will use actual VP9 encoded data
    vec![
        0x82,        // Frame marker + profile
        0x49, 0x83,  // Sync code
        0x42, 0x00,  // Width/height coded
    ]
}
