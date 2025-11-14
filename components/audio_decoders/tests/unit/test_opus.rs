//! Unit tests for OpusDecoder
//!
//! Tests for Opus audio decoder implementation

use cortenbrowser_audio_decoders::OpusDecoder;
use cortenbrowser_shared_types::{
    AudioBuffer, AudioCodec, AudioDecoder, AudioPacket, MediaError, OpusApplication,
};
use std::time::Duration;

#[test]
fn test_opus_decoder_new_creates_decoder() {
    /**
     * Given valid sample rate and channel count
     * When creating new OpusDecoder
     * Then decoder is created successfully
     */
    // Given
    let sample_rate = 48000u32;
    let channels = 2u8;

    // When
    let result = OpusDecoder::new(sample_rate, channels);

    // Then
    assert!(result.is_ok(), "OpusDecoder creation should succeed");
}

#[test]
fn test_opus_decoder_rejects_invalid_sample_rate() {
    /**
     * Given invalid sample rate (not 8000, 12000, 16000, 24000, or 48000)
     * When creating new OpusDecoder
     * Then decoder creation fails with error
     */
    // Given
    let invalid_sample_rate = 44100u32; // Not valid for Opus
    let channels = 2u8;

    // When
    let result = OpusDecoder::new(invalid_sample_rate, channels);

    // Then
    assert!(
        result.is_err(),
        "OpusDecoder should reject invalid sample rate"
    );
    if let Err(MediaError::CodecError { details }) = result {
        assert!(details.contains("sample rate") || details.contains("48000"));
    } else {
        panic!("Expected CodecError for invalid sample rate");
    }
}

#[test]
fn test_opus_decoder_rejects_invalid_channels() {
    /**
     * Given invalid channel count (> 2 for basic stereo)
     * When creating new OpusDecoder
     * Then decoder creation fails with error
     */
    // Given
    let sample_rate = 48000u32;
    let invalid_channels = 0u8;

    // When
    let result = OpusDecoder::new(sample_rate, invalid_channels);

    // Then
    assert!(
        result.is_err(),
        "OpusDecoder should reject invalid channels"
    );
}

#[test]
fn test_opus_decoder_decode_opus_packet() {
    /**
     * Given a valid Opus decoder and encoded Opus packet
     * When decoding the packet
     * Then decoded audio buffer is returned with correct properties
     */
    // Given
    let sample_rate = 48000u32;
    let channels = 2u8;
    let mut decoder = OpusDecoder::new(sample_rate, channels).expect("Decoder should be created");

    // Create a test packet with valid Opus data (silence frame)
    // This is a minimal valid Opus packet (TOC byte + empty payload for silence)
    let packet = AudioPacket {
        data: vec![0xFC], // TOC byte for Opus stereo, 20ms frame
        pts: Some(0),
        dts: Some(0),
    };

    // When
    let result = decoder.decode(&packet);

    // Then
    assert!(result.is_ok(), "Decoding should succeed");
    let buffer = result.unwrap();
    assert_eq!(buffer.sample_rate, sample_rate);
    assert_eq!(buffer.channels, channels);
    assert!(buffer.samples.len() > 0, "Buffer should contain samples");
}

#[test]
fn test_opus_decoder_maintains_state_across_packets() {
    /**
     * Given an Opus decoder
     * When decoding multiple sequential packets
     * Then each packet is decoded independently with consistent output
     */
    // Given
    let sample_rate = 48000u32;
    let channels = 2u8;
    let mut decoder = OpusDecoder::new(sample_rate, channels).expect("Decoder should be created");

    let packet1 = AudioPacket {
        data: vec![0xFC],
        pts: Some(0),
        dts: Some(0),
    };
    let packet2 = AudioPacket {
        data: vec![0xFC],
        pts: Some(960), // Next frame
        dts: Some(960),
    };

    // When
    let result1 = decoder.decode(&packet1);
    let result2 = decoder.decode(&packet2);

    // Then
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    let buffer1 = result1.unwrap();
    let buffer2 = result2.unwrap();
    assert_eq!(buffer1.sample_rate, buffer2.sample_rate);
    assert_eq!(buffer1.channels, buffer2.channels);
}

#[test]
fn test_opus_decoder_handles_empty_packet() {
    /**
     * Given an Opus decoder and empty packet
     * When decoding the empty packet
     * Then decoding fails with appropriate error
     */
    // Given
    let sample_rate = 48000u32;
    let channels = 2u8;
    let mut decoder = OpusDecoder::new(sample_rate, channels).expect("Decoder should be created");

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
fn test_opus_decoder_flush_returns_empty() {
    /**
     * Given an Opus decoder
     * When flushing the decoder
     * Then empty vector is returned (Opus doesn't buffer frames)
     */
    // Given
    let sample_rate = 48000u32;
    let channels = 2u8;
    let mut decoder = OpusDecoder::new(sample_rate, channels).expect("Decoder should be created");

    // When
    let result = decoder.flush();

    // Then
    assert!(result.is_ok());
    let buffers = result.unwrap();
    assert_eq!(buffers.len(), 0, "Opus decoder should not buffer frames");
}
