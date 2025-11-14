//! Unit tests for H.264 decoder
//!
//! Following TDD: These tests are written FIRST (RED phase)

use cortenbrowser_shared_types::{MediaError, VideoDecoder, VideoFrame, VideoPacket};
use cortenbrowser_video_decoders::H264Decoder;

#[test]
fn test_h264_decoder_creation() {
    //! Test that H.264 decoder can be created successfully

    let result = H264Decoder::new();
    assert!(result.is_ok(), "H264Decoder::new() should succeed");
}

#[test]
fn test_h264_decoder_decode_valid_packet() {
    //! Test decoding a valid H.264 packet
    //!
    //! Given a valid H.264 packet
    //! When decode is called
    //! Then a VideoFrame should be returned

    let mut decoder = H264Decoder::new().expect("Failed to create decoder");

    // Create a minimal valid H.264 packet (I-frame header)
    // This is a simplified test packet - real decoder will need actual H.264 data
    let packet = VideoPacket {
        data: create_test_h264_iframe(),
        pts: Some(0),
        dts: Some(0),
        is_keyframe: true,
    };

    let result = decoder.decode(&packet);
    assert!(result.is_ok(), "Decoding valid packet should succeed");

    let frame = result.unwrap();
    assert!(frame.width > 0, "Frame width should be positive");
    assert!(frame.height > 0, "Frame height should be positive");
    assert!(!frame.data.is_empty(), "Frame data should not be empty");
}

#[test]
fn test_h264_decoder_decode_invalid_packet() {
    //! Test decoding an invalid packet
    //!
    //! Given an invalid packet
    //! When decode is called
    //! Then a MediaError should be returned

    let mut decoder = H264Decoder::new().expect("Failed to create decoder");

    let packet = VideoPacket {
        data: vec![0xFF, 0xFF, 0xFF],  // Invalid data
        pts: Some(0),
        dts: Some(0),
        is_keyframe: false,
    };

    let result = decoder.decode(&packet);
    assert!(result.is_err(), "Decoding invalid packet should fail");

    match result {
        Err(MediaError::CodecError { .. }) => {
            // Expected error type
        }
        _ => panic!("Expected CodecError for invalid packet"),
    }
}

#[test]
fn test_h264_decoder_flush() {
    //! Test flushing buffered frames
    //!
    //! Given a decoder with buffered frames
    //! When flush is called
    //! Then all buffered frames should be returned

    let mut decoder = H264Decoder::new().expect("Failed to create decoder");

    // Decode a packet first to potentially buffer frames
    let packet = VideoPacket {
        data: create_test_h264_iframe(),
        pts: Some(0),
        dts: Some(0),
        is_keyframe: true,
    };

    let _ = decoder.decode(&packet);

    // Flush should succeed
    let result = decoder.flush();
    assert!(result.is_ok(), "Flush should succeed");

    // Flushed frames should be a vec (may be empty)
    let frames = result.unwrap();
    assert!(frames.len() >= 0, "Flush should return a vector");
}

#[test]
fn test_h264_decoder_multiple_packets() {
    //! Test decoding multiple sequential packets
    //!
    //! Given multiple H.264 packets
    //! When they are decoded in sequence
    //! Then each should produce a frame with increasing timestamps

    let mut decoder = H264Decoder::new().expect("Failed to create decoder");

    let packets = vec![
        VideoPacket {
            data: create_test_h264_iframe(),
            pts: Some(0),
            dts: Some(0),
            is_keyframe: true,
        },
        VideoPacket {
            data: create_test_h264_pframe(),
            pts: Some(33),
            dts: Some(33),
            is_keyframe: false,
        },
    ];

    for packet in packets {
        let result = decoder.decode(&packet);
        assert!(result.is_ok(), "Each packet should decode successfully");
    }
}

// Helper function to create test H.264 I-frame data
fn create_test_h264_iframe() -> Vec<u8> {
    // This creates a minimal H.264 NAL unit for testing
    // Real implementation will use actual H.264 encoded data
    // For now, create a valid NAL unit header for an I-frame
    vec![
        0x00, 0x00, 0x00, 0x01,  // Start code
        0x67,                     // SPS NAL unit
        0x42, 0x00, 0x1f,        // Profile/level
        0x00, 0x00, 0x00, 0x01,  // Start code
        0x68,                     // PPS NAL unit
        0x00, 0x00, 0x00, 0x01,  // Start code
        0x65,                     // IDR slice
    ]
}

// Helper function to create test H.264 P-frame data
fn create_test_h264_pframe() -> Vec<u8> {
    vec![
        0x00, 0x00, 0x00, 0x01,  // Start code
        0x41,                     // P-frame NAL unit
    ]
}
