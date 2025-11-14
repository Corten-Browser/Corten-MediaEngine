//! Decoder factory for creating codec-specific decoders
//!
//! The factory pattern allows creation of decoders based on codec type
//! without needing to know the specific implementation.

use cortenbrowser_shared_types::{MediaError, VideoCodec, VideoDecoder};

#[cfg(feature = "h264")]
use crate::H264Decoder;

#[cfg(feature = "vp9")]
use crate::VP9Decoder;

#[cfg(feature = "av1")]
use crate::AV1Decoder;

/// Factory for creating video decoders based on codec type
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_video_decoders::DecoderFactory;
/// use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};
///
/// let codec = VideoCodec::H264 {
///     profile: H264Profile::High,
///     level: H264Level::Level4_1,
///     hardware_accel: false,
/// };
///
/// let decoder = DecoderFactory::create_decoder(codec).unwrap();
/// ```
pub struct DecoderFactory;

impl DecoderFactory {
    /// Creates a decoder for the specified codec
    ///
    /// # Arguments
    ///
    /// * `codec` - The video codec to create a decoder for
    ///
    /// # Returns
    ///
    /// Returns a boxed `VideoDecoder` trait object for the specified codec,
    /// or a `MediaError::UnsupportedFormat` if the codec is not supported.
    ///
    /// # Errors
    ///
    /// - `UnsupportedFormat` - The codec is not supported (e.g., Theora, H.265, VP8)
    /// - `CodecError` - Failed to initialize the decoder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cortenbrowser_video_decoders::DecoderFactory;
    /// use cortenbrowser_shared_types::{VideoCodec, VP9Profile};
    ///
    /// let codec = VideoCodec::VP9 {
    ///     profile: VP9Profile::Profile0,
    /// };
    ///
    /// let decoder = DecoderFactory::create_decoder(codec)
    ///     .expect("Failed to create decoder");
    /// ```
    pub fn create_decoder(codec: VideoCodec) -> Result<Box<dyn VideoDecoder>, MediaError> {
        match codec {
            #[cfg(feature = "h264")]
            VideoCodec::H264 { .. } => {
                let decoder = H264Decoder::new()?;
                Ok(Box::new(decoder))
            }
            #[cfg(not(feature = "h264"))]
            VideoCodec::H264 { .. } => Err(MediaError::UnsupportedFormat {
                format: "H.264 support not enabled (compile with --features h264)".to_string(),
            }),

            #[cfg(feature = "vp9")]
            VideoCodec::VP9 { .. } => {
                let decoder = VP9Decoder::new()?;
                Ok(Box::new(decoder))
            }
            #[cfg(not(feature = "vp9"))]
            VideoCodec::VP9 { .. } => Err(MediaError::UnsupportedFormat {
                format: "VP9 support not enabled (compile with --features vp9)".to_string(),
            }),

            #[cfg(feature = "av1")]
            VideoCodec::AV1 { .. } => {
                let decoder = AV1Decoder::new()?;
                Ok(Box::new(decoder))
            }
            #[cfg(not(feature = "av1"))]
            VideoCodec::AV1 { .. } => Err(MediaError::UnsupportedFormat {
                format: "AV1 support not enabled (compile with --features av1)".to_string(),
            }),

            VideoCodec::H265 { .. } => Err(MediaError::UnsupportedFormat {
                format: "H.265/HEVC is not yet supported".to_string(),
            }),
            VideoCodec::VP8 => Err(MediaError::UnsupportedFormat {
                format: "VP8 is not yet supported".to_string(),
            }),
            VideoCodec::Theora => Err(MediaError::UnsupportedFormat {
                format: "Theora codec is not supported".to_string(),
            }),
        }
    }

    /// Returns a list of supported codecs
    ///
    /// The returned list depends on which codec features are enabled during compilation.
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_video_decoders::DecoderFactory;
    ///
    /// let supported = DecoderFactory::supported_codecs();
    /// // With default features, all codecs are supported
    /// assert!(!supported.is_empty());
    ///
    /// // Check for specific codecs based on enabled features
    /// #[cfg(feature = "h264")]
    /// assert!(supported.contains(&"H.264"));
    /// ```
    #[allow(clippy::vec_init_then_push)] // Conditional compilation makes vec![] impractical
    pub fn supported_codecs() -> Vec<&'static str> {
        let mut codecs = Vec::new();

        #[cfg(feature = "h264")]
        codecs.push("H.264");

        #[cfg(feature = "vp9")]
        codecs.push("VP9");

        #[cfg(feature = "av1")]
        codecs.push("AV1");

        codecs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cortenbrowser_shared_types::{H264Level, H264Profile};

    #[cfg(feature = "h264")]
    #[test]
    fn test_create_h264_decoder() {
        let codec = VideoCodec::H264 {
            profile: H264Profile::High,
            level: H264Level::Level4_1,
            hardware_accel: false,
        };

        let result = DecoderFactory::create_decoder(codec);
        assert!(result.is_ok(), "Should create H.264 decoder");
    }

    #[cfg(feature = "vp9")]
    #[test]
    fn test_create_vp9_decoder() {
        use cortenbrowser_shared_types::VP9Profile;

        let codec = VideoCodec::VP9 {
            profile: VP9Profile::Profile0,
        };

        let result = DecoderFactory::create_decoder(codec);
        assert!(result.is_ok(), "Should create VP9 decoder");
    }

    #[cfg(feature = "av1")]
    #[test]
    fn test_create_av1_decoder() {
        use cortenbrowser_shared_types::{AV1Level, AV1Profile};

        let codec = VideoCodec::AV1 {
            profile: AV1Profile::Main,
            level: AV1Level::Level4_0,
        };

        let result = DecoderFactory::create_decoder(codec);
        assert!(result.is_ok(), "Should create AV1 decoder");
    }

    #[test]
    fn test_unsupported_codec() {
        let codec = VideoCodec::Theora;

        let result = DecoderFactory::create_decoder(codec);
        assert!(result.is_err(), "Theora should be unsupported");

        match result {
            Err(MediaError::UnsupportedFormat { .. }) => (),
            _ => panic!("Expected UnsupportedFormat error"),
        }
    }

    #[test]
    fn test_supported_codecs_list() {
        let supported = DecoderFactory::supported_codecs();

        // Check based on enabled features
        #[cfg(feature = "h264")]
        assert!(supported.contains(&"H.264"));

        #[cfg(feature = "vp9")]
        assert!(supported.contains(&"VP9"));

        #[cfg(feature = "av1")]
        assert!(supported.contains(&"AV1"));
    }
}
