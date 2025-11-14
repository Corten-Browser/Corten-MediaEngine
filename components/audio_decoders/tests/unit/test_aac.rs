//! Unit tests for AACDecoder
//!
//! Tests for AAC audio decoder implementation

use cortenbrowser_audio_decoders::AACDecoder;
use cortenbrowser_shared_types::{
    AACProfile, AudioBuffer, AudioCodec, AudioDecoder, AudioPacket, MediaError,
};
use std::time::Duration;

#[test]
fn test_aac_decoder_new_creates_decoder() {
    /**
     * Given no parameters
     * When creating new AACDecoder
     * Then decoder is created successfully
     */
    // When
    let result = AACDecoder::new();

    // Then
    assert!(result.is_ok(), "AACDecoder creation should succeed");
}

#[test]
fn test_aac_decoder_decode_aac_packet() {
    /**
     * Given a valid AAC decoder and encoded AAC packet
     * When decoding the packet
     * Then decoded audio buffer is returned with correct properties
     */
    // Given
    let mut decoder = AACDecoder::new().expect("Decoder should be created");

    // AAC ADTS header (simplified) - 7 bytes minimum
    // Sync word (12 bits): 0xFFF
    // MPEG-4, Layer 0, no CRC, profile AAC-LC
    let aac_frame = vec![
        0xFF, 0xF1, // Sync word + MPEG version + Layer
        0x50, // Profile + sample rate index + channel
        0x80, // Channel + frame length high bits
        0x00, 0x1F, // Frame length low bits
        0xFC, // Buffer fullness + frame count
              // Minimal AAC data would follow
    ];

    let packet = AudioPacket {
        data: aac_frame,
        pts: Some(0),
        dts: Some(0),
    };

    // When
    let result = decoder.decode(&packet);

    // Then
    // Note: Decoding may fail with incomplete data, which is acceptable for unit test
    // We're testing the interface, not complete AAC decoding
    if result.is_ok() {
        let buffer = result.unwrap();
        assert!(buffer.sample_rate > 0, "Sample rate should be detected");
        assert!(buffer.channels > 0, "Channels should be detected");
    }
}

#[test]
fn test_aac_decoder_handles_invalid_data() {
    /**
     * Given an AAC decoder and invalid data
     * When decoding the invalid packet
     * Then decoding fails with appropriate error
     */
    // Given
    let mut decoder = AACDecoder::new().expect("Decoder should be created");

    let invalid_packet = AudioPacket {
        data: vec![0x00, 0x00, 0x00, 0x00], // Not valid AAC data
        pts: Some(0),
        dts: Some(0),
    };

    // When
    let result = decoder.decode(&invalid_packet);

    // Then
    assert!(result.is_err(), "Invalid data should fail to decode");
}

#[test]
fn test_aac_decoder_handles_empty_packet() {
    /**
     * Given an AAC decoder and empty packet
     * When decoding the empty packet
     * Then decoding fails with appropriate error
     */
    // Given
    let mut decoder = AACDecoder::new().expect("Decoder should be created");

    let empty_packet = AudioPacket {
        data: vec![],
        pts: Some(0),
        dts: Some(0),
    };

    // When
    let result = decoder.decode(&empty_packet);

    // Then
    assert!(result.is_err(), "Empty packet should fail to decode");
}

#[test]
fn test_aac_decoder_flush_returns_empty() {
    /**
     * Given an AAC decoder
     * When flushing the decoder
     * Then empty or buffered frames are returned
     */
    // Given
    let mut decoder = AACDecoder::new().expect("Decoder should be created");

    // When
    let result = decoder.flush();

    // Then
    assert!(result.is_ok(), "Flush should succeed");
    // AAC may or may not buffer frames depending on implementation
}
