//! Unit tests for AV1 decoder
//!
//! Following TDD: These tests are written FIRST (RED phase)

use cortenbrowser_shared_types::{MediaError, VideoDecoder, VideoPacket};
use cortenbrowser_video_decoders::AV1Decoder;

#[test]
fn test_av1_decoder_creation() {
    //! Test that AV1 decoder can be created successfully

    let result = AV1Decoder::new();
    assert!(result.is_ok(), "AV1Decoder::new() should succeed");
}

#[test]
fn test_av1_decoder_decode_valid_packet() {
    //! Test decoding a valid AV1 packet
    //!
    //! Given a valid AV1 packet
    //! When decode is called
    //! Then a VideoFrame should be returned

    let mut decoder = AV1Decoder::new().expect("Failed to create decoder");

    let packet = VideoPacket {
        data: create_test_av1_frame(),
        pts: Some(0),
        dts: Some(0),
        is_keyframe: true,
    };

    let result = decoder.decode(&packet);
    assert!(result.is_ok(), "Decoding valid AV1 packet should succeed");

    let frame = result.unwrap();
    assert!(frame.width > 0, "Frame width should be positive");
    assert!(frame.height > 0, "Frame height should be positive");
}

#[test]
fn test_av1_decoder_decode_invalid_packet() {
    //! Test decoding an invalid packet
    //!
    //! Given an invalid AV1 packet
    //! When decode is called
    //! Then a CodecError should be returned

    let mut decoder = AV1Decoder::new().expect("Failed to create decoder");

    let packet = VideoPacket {
        data: vec![0xFF, 0xFF, 0xFF],
        pts: Some(0),
        dts: Some(0),
        is_keyframe: false,
    };

    let result = decoder.decode(&packet);
    assert!(result.is_err(), "Decoding invalid AV1 packet should fail");
}

#[test]
fn test_av1_decoder_flush() {
    //! Test flushing AV1 decoder

    let mut decoder = AV1Decoder::new().expect("Failed to create decoder");

    let result = decoder.flush();
    assert!(result.is_ok(), "AV1 flush should succeed");
}

// Helper function to create test AV1 frame data
fn create_test_av1_frame() -> Vec<u8> {
    // Minimal AV1 keyframe for testing
    // Real implementation will use actual AV1 encoded data
    vec![
        0x12, 0x00,  // OBU header
        0x0A,        // Sequence header OBU
    ]
}
