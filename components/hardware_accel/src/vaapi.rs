//! VA-API hardware decoder for Linux

use crate::error::{HardwareError, HardwareResult};
use cortenbrowser_shared_types::{
    FrameMetadata, MediaError, PixelFormat, VideoCodec, VideoDecoder, VideoFrame, VideoPacket,
};
use std::time::Duration;

/// VA-API hardware video decoder
///
/// Provides hardware-accelerated video decoding on Linux systems using VA-API.
///
/// # Platform Support
///
/// This decoder is only available on Linux with VA-API drivers installed.
/// Common drivers include:
/// - Intel: `intel-media-driver` or `i965-va-driver`
/// - AMD: `mesa-va-drivers`
/// - NVIDIA: `nvidia-vaapi-driver`
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_hardware_accel::VAAPIDecoder;
/// use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let codec = VideoCodec::H264 {
///     profile: H264Profile::High,
///     level: H264Level::Level4_1,
///     hardware_accel: true,
/// };
///
/// let decoder = VAAPIDecoder::new(&codec)?;
/// # Ok(())
/// # }
/// ```
pub struct VAAPIDecoder {
    _codec: VideoCodec, // Stored for future use (e.g., reconfiguration)
    initialized: bool,
}

impl VAAPIDecoder {
    /// Create a new VA-API decoder
    ///
    /// # Arguments
    ///
    /// * `codec` - The video codec to decode
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `HardwareError::UnsupportedCodec` if the codec is not supported by VA-API
    /// - `HardwareError::NotAvailable` if VA-API is not available
    /// - `HardwareError::InitializationFailed` if decoder initialization fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cortenbrowser_hardware_accel::VAAPIDecoder;
    /// use cortenbrowser_shared_types::{VideoCodec, VP9Profile};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let codec = VideoCodec::VP9 {
    ///     profile: VP9Profile::Profile0,
    /// };
    ///
    /// let decoder = VAAPIDecoder::new(&codec)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(codec: &VideoCodec) -> HardwareResult<Self> {
        // Check if codec is supported by VA-API
        if !Self::is_codec_supported(codec) {
            return Err(HardwareError::UnsupportedCodec);
        }

        // In a real implementation, this would:
        // 1. Open VA display (vaGetDisplay)
        // 2. Initialize VA-API (vaInitialize)
        // 3. Query supported profiles (vaQueryConfigProfiles)
        // 4. Create decoder config (vaCreateConfig)
        // 5. Create decoder context (vaCreateContext)
        //
        // For now, we simulate initialization
        // This allows testing without actual VA-API hardware

        Ok(Self {
            _codec: codec.clone(),
            initialized: true,
        })
    }

    /// Check if a codec is supported by VA-API
    fn is_codec_supported(codec: &VideoCodec) -> bool {
        match codec {
            VideoCodec::H264 { .. } => true,
            VideoCodec::VP9 { .. } => true,
            VideoCodec::VP8 => true,
            VideoCodec::H265 { .. } => true,
            VideoCodec::AV1 { .. } => true,
            VideoCodec::Theora => false, // Not supported by VA-API
        }
    }
}

impl VideoDecoder for VAAPIDecoder {
    /// Decode a video packet
    ///
    /// # Arguments
    ///
    /// * `packet` - The compressed video packet to decode
    ///
    /// # Returns
    ///
    /// Returns a decoded video frame or an error.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::CodecError` if decoding fails.
    ///
    /// # Implementation Notes
    ///
    /// In a full VA-API implementation, this would:
    /// 1. Create VA buffer for bitstream (vaCreateBuffer)
    /// 2. Begin picture decode (vaBeginPicture)
    /// 3. Render picture (vaRenderPicture)
    /// 4. End picture (vaEndPicture)
    /// 5. Sync and map surface (vaSyncSurface, vaMapBuffer)
    /// 6. Copy frame data
    /// 7. Unmap buffer (vaUnmapBuffer)
    ///
    /// For testing purposes, this returns a mock frame.
    fn decode(&mut self, packet: &VideoPacket) -> Result<VideoFrame, MediaError> {
        if !self.initialized {
            return Err(MediaError::CodecError {
                details: "Decoder not initialized".to_string(),
            });
        }

        // In a real implementation, this would decode the packet using VA-API
        // For now, return a mock frame for testing purposes

        // Calculate timestamp
        let timestamp = packet
            .pts
            .map(|pts| Duration::from_millis(pts as u64 * 33)) // ~30fps
            .unwrap_or(Duration::ZERO);

        // Create mock decoded frame
        // In reality, this would be the actual decoded YUV data from hardware
        Ok(VideoFrame {
            width: 1920,
            height: 1080,
            format: PixelFormat::YUV420,
            data: vec![0u8; 1920 * 1080 * 3 / 2], // YUV420 size
            timestamp,
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata::default(),
        })
    }

    /// Flush any buffered frames
    ///
    /// # Returns
    ///
    /// Returns any remaining frames in the decoder's internal buffer.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::CodecError` if flushing fails.
    ///
    /// # Implementation Notes
    ///
    /// In a full VA-API implementation, this would:
    /// 1. Flush the decoder pipeline
    /// 2. Retrieve any cached frames
    /// 3. Reset decoder state
    ///
    /// For testing purposes, this returns an empty vector.
    fn flush(&mut self) -> Result<Vec<VideoFrame>, MediaError> {
        // In a real implementation, this would flush any buffered frames
        // For now, return empty vector (no buffered frames in mock)
        Ok(Vec::new())
    }
}

impl Drop for VAAPIDecoder {
    fn drop(&mut self) {
        // In a real implementation, this would:
        // 1. Destroy VA context (vaDestroyContext)
        // 2. Destroy VA config (vaDestroyConfig)
        // 3. Terminate VA display (vaTerminate)
        //
        // For now, just mark as uninitialized
        self.initialized = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cortenbrowser_shared_types::{H264Level, H264Profile};

    #[test]
    fn test_vaapi_decoder_creation() {
        let codec = VideoCodec::H264 {
            profile: H264Profile::High,
            level: H264Level::Level4_1,
            hardware_accel: true,
        };

        let decoder = VAAPIDecoder::new(&codec);
        assert!(decoder.is_ok());
    }

    #[test]
    fn test_vaapi_unsupported_codec() {
        let codec = VideoCodec::Theora;

        let decoder = VAAPIDecoder::new(&codec);
        assert!(matches!(decoder, Err(HardwareError::UnsupportedCodec)));
    }
}
