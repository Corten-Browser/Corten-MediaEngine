//! Unit tests for DecoderFactory
//!
//! Tests for the audio decoder factory

use cortenbrowser_audio_decoders::DecoderFactory;
use cortenbrowser_shared_types::{
    AACProfile, AudioCodec, MP3Layer, MediaError, OpusApplication, PCMFormat,
};

#[test]
fn test_factory_creates_opus_decoder() {
    /**
     * Given an Opus codec specification
     * When creating decoder via factory
     * Then OpusDecoder is returned
     */
    // Given
    let codec = AudioCodec::Opus {
        sample_rate: 48000,
        channels: 2,
        application: OpusApplication::Audio,
    };

    // When
    let result = DecoderFactory::create_decoder(codec);

    // Then
    assert!(result.is_ok(), "Factory should create Opus decoder");
}

#[test]
fn test_factory_creates_mp3_decoder() {
    /**
     * Given an MP3 codec specification
     * When creating decoder via factory
     * Then MP3Decoder is returned
     */
    // Given
    let codec = AudioCodec::MP3 {
        layer: MP3Layer::Layer3,
        bitrate: 128000,
    };

    // When
    let result = DecoderFactory::create_decoder(codec);

    // Then
    assert!(result.is_ok(), "Factory should create MP3 decoder");
}

#[test]
fn test_factory_creates_aac_decoder() {
    /**
     * Given an AAC codec specification
     * When creating decoder via factory
     * Then AACDecoder is returned
     */
    // Given
    let codec = AudioCodec::AAC {
        profile: AACProfile::LC,
        sample_rate: 48000,
        channels: 2,
    };

    // When
    let result = DecoderFactory::create_decoder(codec);

    // Then
    assert!(result.is_ok(), "Factory should create AAC decoder");
}

#[test]
fn test_factory_rejects_unsupported_codec() {
    /**
     * Given an unsupported codec (Vorbis, FLAC, PCM)
     * When creating decoder via factory
     * Then error is returned indicating unsupported format
     */
    // Given - Vorbis codec
    let vorbis_codec = AudioCodec::Vorbis;

    // When
    let result = DecoderFactory::create_decoder(vorbis_codec);

    // Then
    assert!(result.is_err(), "Factory should reject Vorbis for now");
    if let Err(MediaError::UnsupportedFormat { format }) = result {
        assert!(format.contains("Vorbis") || format.contains("supported"));
    }
}

#[test]
fn test_factory_rejects_flac() {
    /**
     * Given FLAC codec
     * When creating decoder via factory
     * Then error is returned (FLAC not implemented)
     */
    // Given
    let codec = AudioCodec::FLAC;

    // When
    let result = DecoderFactory::create_decoder(codec);

    // Then
    assert!(result.is_err(), "Factory should reject FLAC");
}

#[test]
fn test_factory_rejects_pcm() {
    /**
     * Given PCM codec
     * When creating decoder via factory
     * Then error is returned (PCM doesn't need decoding)
     */
    // Given
    let codec = AudioCodec::PCM {
        format: PCMFormat::F32LE,
        sample_rate: 48000,
        channels: 2,
    };

    // When
    let result = DecoderFactory::create_decoder(codec);

    // Then
    assert!(
        result.is_err(),
        "Factory should reject PCM (no decoding needed)"
    );
}

#[test]
fn test_factory_handles_opus_invalid_params() {
    /**
     * Given Opus codec with invalid parameters
     * When creating decoder via factory
     * Then error is returned
     */
    // Given - invalid sample rate for Opus
    let codec = AudioCodec::Opus {
        sample_rate: 44100, // Not valid for Opus
        channels: 2,
        application: OpusApplication::Audio,
    };

    // When
    let result = DecoderFactory::create_decoder(codec);

    // Then
    assert!(
        result.is_err(),
        "Factory should reject invalid Opus parameters"
    );
}
