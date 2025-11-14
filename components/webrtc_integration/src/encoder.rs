//! WebRTC encoder wrapper
//!
//! Provides a wrapper around video encoders for WebRTC streaming.

use cortenbrowser_shared_types::{VideoCodec, VideoFrame, MediaError};

/// Encoder configuration
///
/// Specifies encoding parameters for video encoding.
///
/// # Examples
///
/// ```
/// use cortenbrowser_webrtc_integration::EncoderConfig;
///
/// let config = EncoderConfig {
///     bitrate: 2_000_000,  // 2 Mbps
///     framerate: 30,
///     keyframe_interval: 60,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncoderConfig {
    /// Target bitrate in bits per second
    pub bitrate: u32,
    /// Target framerate in frames per second
    pub framerate: u32,
    /// Keyframe interval in frames
    pub keyframe_interval: u32,
}

/// WebRTC video encoder
///
/// Wraps video encoders for WebRTC streaming. Currently a stub implementation
/// that validates inputs and returns mock encoded data.
///
/// # Examples
///
/// ```
/// use cortenbrowser_webrtc_integration::{WebRTCEncoder, EncoderConfig};
/// use cortenbrowser_shared_types::{VideoCodec, VideoFrame, PixelFormat, H264Profile, H264Level, FrameMetadata};
/// use std::time::Duration;
///
/// let codec = VideoCodec::H264 {
///     profile: H264Profile::Main,
///     level: H264Level::Level4_0,
///     hardware_accel: false,
/// };
///
/// let config = EncoderConfig {
///     bitrate: 1_000_000,
///     framerate: 30,
///     keyframe_interval: 30,
/// };
///
/// let encoder = WebRTCEncoder::new(codec, config).unwrap();
///
/// let frame = VideoFrame {
///     width: 640,
///     height: 480,
///     format: PixelFormat::YUV420,
///     data: vec![0u8; 640 * 480 * 3 / 2],
///     timestamp: Duration::from_millis(0),
///     duration: Some(Duration::from_millis(33)),
///     metadata: FrameMetadata::default(),
/// };
///
/// let encoded = encoder.encode(&frame).unwrap();
/// assert!(!encoded.is_empty());
/// ```
pub struct WebRTCEncoder {
    codec: VideoCodec,
    config: EncoderConfig,
    frame_count: std::cell::Cell<u32>,
}

impl WebRTCEncoder {
    /// Create a new WebRTC encoder
    ///
    /// # Arguments
    ///
    /// * `codec` - The video codec to use
    /// * `config` - Encoder configuration
    ///
    /// # Errors
    ///
    /// Returns `MediaError::CodecError` if:
    /// - Bitrate is zero
    /// - Framerate is zero
    /// - Codec is not supported (currently stub supports H264, VP8, VP9)
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_webrtc_integration::{WebRTCEncoder, EncoderConfig};
    /// use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};
    ///
    /// let codec = VideoCodec::H264 {
    ///     profile: H264Profile::High,
    ///     level: H264Level::Level4_1,
    ///     hardware_accel: false,
    /// };
    ///
    /// let config = EncoderConfig {
    ///     bitrate: 2_000_000,
    ///     framerate: 30,
    ///     keyframe_interval: 60,
    /// };
    ///
    /// let encoder = WebRTCEncoder::new(codec, config);
    /// assert!(encoder.is_ok());
    /// ```
    pub fn new(codec: VideoCodec, config: EncoderConfig) -> Result<Self, MediaError> {
        // Validate config
        if config.bitrate == 0 {
            return Err(MediaError::CodecError {
                details: "Bitrate cannot be zero".to_string(),
            });
        }

        if config.framerate == 0 {
            return Err(MediaError::CodecError {
                details: "Framerate cannot be zero".to_string(),
            });
        }

        // Validate codec is supported
        match &codec {
            VideoCodec::H264 { .. } | VideoCodec::VP8 | VideoCodec::VP9 { .. } | VideoCodec::AV1 { .. } => {
                // Supported
            }
            _ => {
                return Err(MediaError::CodecError {
                    details: format!("Codec {:?} not supported for WebRTC", codec),
                });
            }
        }

        Ok(Self {
            codec,
            config,
            frame_count: std::cell::Cell::new(0),
        })
    }

    /// Encode a video frame
    ///
    /// # Arguments
    ///
    /// * `frame` - The video frame to encode
    ///
    /// # Returns
    ///
    /// Encoded frame data as bytes
    ///
    /// # Errors
    ///
    /// Returns `MediaError::CodecError` if:
    /// - Frame dimensions don't match expected data size
    /// - Frame data is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_webrtc_integration::{WebRTCEncoder, EncoderConfig};
    /// use cortenbrowser_shared_types::{VideoCodec, VideoFrame, PixelFormat, FrameMetadata};
    /// use std::time::Duration;
    ///
    /// let encoder = WebRTCEncoder::new(
    ///     VideoCodec::VP8,
    ///     EncoderConfig {
    ///         bitrate: 1_000_000,
    ///         framerate: 30,
    ///         keyframe_interval: 30,
    ///     }
    /// ).unwrap();
    ///
    /// let frame = VideoFrame {
    ///     width: 320,
    ///     height: 240,
    ///     format: PixelFormat::YUV420,
    ///     data: vec![0u8; 320 * 240 * 3 / 2],
    ///     timestamp: Duration::from_millis(0),
    ///     duration: Some(Duration::from_millis(33)),
    ///     metadata: FrameMetadata::default(),
    /// };
    ///
    /// let encoded = encoder.encode(&frame);
    /// assert!(encoded.is_ok());
    /// ```
    pub fn encode(&self, frame: &VideoFrame) -> Result<Vec<u8>, MediaError> {
        // Validate frame data size
        let expected_size = self.calculate_expected_frame_size(frame);
        if frame.data.len() < expected_size {
            return Err(MediaError::CodecError {
                details: format!(
                    "Frame data size {} is less than expected {} for {}x{} {:?}",
                    frame.data.len(),
                    expected_size,
                    frame.width,
                    frame.height,
                    frame.format
                ),
            });
        }

        // Increment frame count
        let count = self.frame_count.get();
        self.frame_count.set(count + 1);

        // Generate mock encoded data
        // In real implementation, this would call actual codec
        let is_keyframe = frame.metadata.is_keyframe || count.is_multiple_of(self.config.keyframe_interval);

        let encoded_size = if is_keyframe {
            // Keyframes are larger
            (frame.data.len() / 4).max(1000)
        } else {
            // P-frames are smaller
            (frame.data.len() / 8).max(500)
        };

        // Create mock encoded data with codec-specific marker
        let mut encoded = Vec::with_capacity(encoded_size + 4);

        // Add codec marker (for testing/debugging)
        match &self.codec {
            VideoCodec::H264 { .. } => encoded.extend_from_slice(b"H264"),
            VideoCodec::VP8 => encoded.extend_from_slice(b"VP8\0"),
            VideoCodec::VP9 { .. } => encoded.extend_from_slice(b"VP9\0"),
            VideoCodec::AV1 { .. } => encoded.extend_from_slice(b"AV1\0"),
            _ => encoded.extend_from_slice(b"UNKN"),
        }

        // Add mock compressed data
        encoded.resize(encoded_size, if is_keyframe { 0xFF } else { 0xAA });

        Ok(encoded)
    }

    /// Calculate expected frame size for validation
    fn calculate_expected_frame_size(&self, frame: &VideoFrame) -> usize {
        use cortenbrowser_shared_types::PixelFormat;

        match frame.format {
            PixelFormat::YUV420 => {
                // YUV420: Y plane + U plane (1/4) + V plane (1/4)
                // = width * height * 3/2
                (frame.width as usize * frame.height as usize * 3) / 2
            }
            PixelFormat::YUV422 => {
                // YUV422: Y plane + U plane (1/2) + V plane (1/2)
                // = width * height * 2
                frame.width as usize * frame.height as usize * 2
            }
            PixelFormat::YUV444 => {
                // YUV444: Y plane + U plane + V plane
                // = width * height * 3
                frame.width as usize * frame.height as usize * 3
            }
            PixelFormat::NV12 => {
                // NV12: Y plane + interleaved UV plane (1/2)
                // = width * height * 3/2
                (frame.width as usize * frame.height as usize * 3) / 2
            }
            PixelFormat::RGB24 => {
                // RGB: 3 bytes per pixel
                frame.width as usize * frame.height as usize * 3
            }
            PixelFormat::RGBA32 => {
                // RGBA: 4 bytes per pixel
                frame.width as usize * frame.height as usize * 4
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cortenbrowser_shared_types::{H264Profile, H264Level, PixelFormat, FrameMetadata};
    use std::time::Duration;

    #[test]
    fn test_encoder_config_validation() {
        let codec = VideoCodec::H264 {
            profile: H264Profile::Main,
            level: H264Level::Level4_0,
            hardware_accel: false,
        };

        // Zero bitrate should fail
        let result = WebRTCEncoder::new(
            codec.clone(),
            EncoderConfig {
                bitrate: 0,
                framerate: 30,
                keyframe_interval: 30,
            },
        );
        assert!(result.is_err());

        // Zero framerate should fail
        let result = WebRTCEncoder::new(
            codec,
            EncoderConfig {
                bitrate: 1_000_000,
                framerate: 0,
                keyframe_interval: 30,
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_encoder_frame_validation() {
        let encoder = WebRTCEncoder::new(
            VideoCodec::VP8,
            EncoderConfig {
                bitrate: 1_000_000,
                framerate: 30,
                keyframe_interval: 30,
            },
        )
        .unwrap();

        // Frame with invalid data size should fail
        let frame = VideoFrame {
            width: 640,
            height: 480,
            format: PixelFormat::YUV420,
            data: vec![0u8; 100], // Too small
            timestamp: Duration::from_millis(0),
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata::default(),
        };

        let result = encoder.encode(&frame);
        assert!(result.is_err());
    }

    #[test]
    fn test_encoder_basic_encoding() {
        let encoder = WebRTCEncoder::new(
            VideoCodec::H264 {
                profile: H264Profile::Main,
                level: H264Level::Level4_0,
                hardware_accel: false,
            },
            EncoderConfig {
                bitrate: 1_000_000,
                framerate: 30,
                keyframe_interval: 30,
            },
        )
        .unwrap();

        let frame = VideoFrame {
            width: 640,
            height: 480,
            format: PixelFormat::YUV420,
            data: vec![0u8; 640 * 480 * 3 / 2],
            timestamp: Duration::from_millis(0),
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata::default(),
        };

        let result = encoder.encode(&frame);
        assert!(result.is_ok());

        let encoded = result.unwrap();
        assert!(!encoded.is_empty());
        assert!(encoded.starts_with(b"H264"));
    }
}
