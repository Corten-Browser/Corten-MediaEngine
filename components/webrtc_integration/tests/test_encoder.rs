//! Unit tests for WebRTC encoder
//!
//! Tests for WebRTCEncoder wrapper

#[cfg(test)]
mod tests {
    use cortenbrowser_webrtc_integration::{WebRTCEncoder, EncoderConfig};
    use cortenbrowser_shared_types::{VideoCodec, VideoFrame, PixelFormat, H264Profile, H264Level, FrameMetadata, MediaError};
    use std::time::Duration;

    #[test]
    fn test_encoder_creation_h264() {
        // RED: Test creating H264 encoder
        let codec = VideoCodec::H264 {
            profile: H264Profile::High,
            level: H264Level::Level4_1,
            hardware_accel: false,
        };

        let config = EncoderConfig {
            bitrate: 2_000_000, // 2 Mbps
            framerate: 30,
            keyframe_interval: 60,
        };

        let encoder = WebRTCEncoder::new(codec, config);
        assert!(encoder.is_ok(), "Should create H264 encoder successfully");
    }

    #[test]
    fn test_encoder_creation_vp8() {
        // RED: Test creating VP8 encoder
        let codec = VideoCodec::VP8;

        let config = EncoderConfig {
            bitrate: 1_500_000, // 1.5 Mbps
            framerate: 30,
            keyframe_interval: 30,
        };

        let encoder = WebRTCEncoder::new(codec, config);
        assert!(encoder.is_ok(), "Should create VP8 encoder successfully");
    }

    #[test]
    fn test_encoder_encode_frame() {
        // RED: Test encoding a video frame
        let codec = VideoCodec::H264 {
            profile: H264Profile::Main,
            level: H264Level::Level4_0,
            hardware_accel: false,
        };

        let config = EncoderConfig {
            bitrate: 1_000_000,
            framerate: 30,
            keyframe_interval: 30,
        };

        let encoder = WebRTCEncoder::new(codec, config).unwrap();

        let frame = VideoFrame {
            width: 640,
            height: 480,
            format: PixelFormat::YUV420,
            data: vec![0u8; 640 * 480 * 3 / 2], // YUV420 size
            timestamp: Duration::from_millis(0),
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata::default(),
        };

        let result = encoder.encode(&frame);
        assert!(result.is_ok(), "Should encode frame successfully");

        let encoded = result.unwrap();
        assert!(!encoded.is_empty(), "Encoded data should not be empty");
    }

    #[test]
    fn test_encoder_multiple_frames() {
        // RED: Test encoding multiple frames in sequence
        let codec = VideoCodec::VP8;
        let config = EncoderConfig {
            bitrate: 1_000_000,
            framerate: 30,
            keyframe_interval: 30,
        };

        let encoder = WebRTCEncoder::new(codec, config).unwrap();

        for i in 0..10 {
            let frame = VideoFrame {
                width: 320,
                height: 240,
                format: PixelFormat::YUV420,
                data: vec![i as u8; 320 * 240 * 3 / 2],
                timestamp: Duration::from_millis(i * 33),
                duration: Some(Duration::from_millis(33)),
                metadata: FrameMetadata::default(),
            };

            let result = encoder.encode(&frame);
            assert!(result.is_ok(), "Frame {} should encode successfully", i);
        }
    }

    #[test]
    fn test_encoder_keyframe_generation() {
        // RED: Test that keyframes are generated at specified intervals
        let codec = VideoCodec::H264 {
            profile: H264Profile::Main,
            level: H264Level::Level4_0,
            hardware_accel: false,
        };

        let config = EncoderConfig {
            bitrate: 1_000_000,
            framerate: 30,
            keyframe_interval: 5, // Keyframe every 5 frames
        };

        let encoder = WebRTCEncoder::new(codec, config).unwrap();

        for i in 0..10 {
            let mut metadata = FrameMetadata::default();
            if i % 5 == 0 {
                // Hint that this should be a keyframe
                metadata.is_keyframe = true;
            }

            let frame = VideoFrame {
                width: 320,
                height: 240,
                format: PixelFormat::YUV420,
                data: vec![0u8; 320 * 240 * 3 / 2],
                timestamp: Duration::from_millis(i * 33),
                duration: Some(Duration::from_millis(33)),
                metadata,
            };

            let result = encoder.encode(&frame);
            assert!(result.is_ok());

            // In real implementation, keyframes would be larger
            // This is a simplified test
        }
    }

    #[test]
    fn test_encoder_invalid_frame_dimensions() {
        // RED: Test encoding frame with invalid dimensions
        let codec = VideoCodec::VP8;
        let config = EncoderConfig {
            bitrate: 1_000_000,
            framerate: 30,
            keyframe_interval: 30,
        };

        let encoder = WebRTCEncoder::new(codec, config).unwrap();

        // Frame with mismatched data size
        let frame = VideoFrame {
            width: 640,
            height: 480,
            format: PixelFormat::YUV420,
            data: vec![0u8; 100], // Too small for 640x480 YUV420
            timestamp: Duration::from_millis(0),
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata::default(),
        };

        let result = encoder.encode(&frame);
        assert!(result.is_err(), "Should fail with invalid frame data");
    }

    #[test]
    fn test_encoder_config_validation() {
        // RED: Test encoder config validation
        let codec = VideoCodec::H264 {
            profile: H264Profile::Main,
            level: H264Level::Level4_0,
            hardware_accel: false,
        };

        // Invalid config: zero bitrate
        let config = EncoderConfig {
            bitrate: 0,
            framerate: 30,
            keyframe_interval: 30,
        };

        let encoder = WebRTCEncoder::new(codec.clone(), config);
        assert!(encoder.is_err(), "Should fail with zero bitrate");

        // Invalid config: zero framerate
        let config = EncoderConfig {
            bitrate: 1_000_000,
            framerate: 0,
            keyframe_interval: 30,
        };

        let encoder = WebRTCEncoder::new(codec, config);
        assert!(encoder.is_err(), "Should fail with zero framerate");
    }
}
