//! Unit tests for VAAPIDecoder (Linux)

#![cfg(target_os = "linux")]

use cortenbrowser_hardware_accel::{HardwareError, VAAPIDecoder};
use cortenbrowser_shared_types::{H264Level, H264Profile, VideoCodec, VideoDecoder, VideoPacket};

#[test]
fn test_vaapi_decoder_new_with_h264() {
    let codec = VideoCodec::H264 {
        profile: H264Profile::High,
        level: H264Level::Level4_1,
        hardware_accel: true,
    };

    let result = VAAPIDecoder::new(&codec);

    // May fail if VA-API not available in test environment
    match result {
        Ok(_decoder) => {
            // VA-API available
        }
        Err(HardwareError::NotAvailable) => {
            // Expected in environments without VA-API
        }
        Err(HardwareError::InitializationFailed) => {
            // Also acceptable
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_vaapi_decoder_new_with_unsupported_codec() {
    let codec = VideoCodec::Theora;

    let result = VAAPIDecoder::new(&codec);

    // Should fail with unsupported codec
    match result {
        Err(HardwareError::UnsupportedCodec) => {
            // Expected
        }
        Err(HardwareError::NotAvailable) => {
            // Also acceptable if VA-API not available
        }
        Ok(_) => panic!("Theora should not be supported by VA-API"),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_vaapi_decoder_implements_video_decoder_trait() {
    // Verify that VAAPIDecoder implements the VideoDecoder trait
    // This is a compile-time check

    let codec = VideoCodec::H264 {
        profile: H264Profile::High,
        level: H264Level::Level4_1,
        hardware_accel: true,
    };

    if let Ok(mut decoder) = VAAPIDecoder::new(&codec) {
        let packet = VideoPacket {
            data: vec![0u8; 100],
            pts: Some(0),
            dts: Some(0),
            is_keyframe: true,
        };

        // decode should compile (trait method exists)
        let _ = decoder.decode(&packet);

        // flush should compile (trait method exists)
        let _ = decoder.flush();
    }
}
