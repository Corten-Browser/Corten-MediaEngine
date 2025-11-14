//! Unit tests for DecoderFactory
//!
//! Following TDD: These tests are written FIRST (RED phase)

use cortenbrowser_shared_types::{
    AV1Level, AV1Profile, H264Level, H264Profile, MediaError, VP9Profile, VideoCodec,
};
use cortenbrowser_video_decoders::DecoderFactory;

#[test]
fn test_factory_create_h264_decoder() {
    //! Test creating H.264 decoder via factory
    //!
    //! Given an H.264 codec specification
    //! When create_decoder is called
    //! Then an H.264 decoder should be returned

    let codec = VideoCodec::H264 {
        profile: H264Profile::High,
        level: H264Level::Level4_1,
        hardware_accel: false,
    };

    let result = DecoderFactory::create_decoder(codec);
    assert!(result.is_ok(), "Factory should create H.264 decoder");
}

#[test]
fn test_factory_create_vp9_decoder() {
    //! Test creating VP9 decoder via factory
    //!
    //! Given a VP9 codec specification
    //! When create_decoder is called
    //! Then a VP9 decoder should be returned

    let codec = VideoCodec::VP9 {
        profile: VP9Profile::Profile0,
    };

    let result = DecoderFactory::create_decoder(codec);
    assert!(result.is_ok(), "Factory should create VP9 decoder");
}

#[test]
fn test_factory_create_av1_decoder() {
    //! Test creating AV1 decoder via factory
    //!
    //! Given an AV1 codec specification
    //! When create_decoder is called
    //! Then an AV1 decoder should be returned

    let codec = VideoCodec::AV1 {
        profile: AV1Profile::Main,
        level: AV1Level::Level4_0,
    };

    let result = DecoderFactory::create_decoder(codec);
    assert!(result.is_ok(), "Factory should create AV1 decoder");
}

#[test]
fn test_factory_unsupported_codec() {
    //! Test that unsupported codecs return an error
    //!
    //! Given an unsupported codec (Theora)
    //! When create_decoder is called
    //! Then an UnsupportedFormat error should be returned

    let codec = VideoCodec::Theora;

    let result = DecoderFactory::create_decoder(codec);
    assert!(result.is_err(), "Unsupported codec should return error");

    match result {
        Err(MediaError::UnsupportedFormat { .. }) => {
            // Expected error type
        }
        _ => panic!("Expected UnsupportedFormat error"),
    }
}
