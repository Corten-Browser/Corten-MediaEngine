//! H.264/AVC video decoder implementation
//!
//! This module provides H.264 decoding using the openh264 library.

use cortenbrowser_shared_types::{
    FrameMetadata, MediaError, PixelFormat, VideoDecoder, VideoFrame, VideoPacket,
};
use openh264::decoder::Decoder as OpenH264Decoder;
use openh264::formats::YUVSource;
use std::time::Duration;

/// H.264 video decoder
///
/// Decodes H.264/AVC video packets into raw video frames using OpenH264.
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_video_decoders::H264Decoder;
/// use cortenbrowser_shared_types::{VideoDecoder, VideoPacket};
///
/// let mut decoder = H264Decoder::new().unwrap();
/// let packet = VideoPacket::default();
/// let frame = decoder.decode(&packet).unwrap();
/// ```
pub struct H264Decoder {
    /// Underlying OpenH264 decoder instance
    decoder: OpenH264Decoder,
    /// Frame sequence counter
    frame_count: u64,
}

impl H264Decoder {
    /// Creates a new H.264 decoder instance
    ///
    /// # Errors
    ///
    /// Returns a `MediaError::CodecError` if decoder initialization fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cortenbrowser_video_decoders::H264Decoder;
    ///
    /// let decoder = H264Decoder::new().expect("Failed to create H.264 decoder");
    /// ```
    pub fn new() -> Result<Self, MediaError> {
        let decoder = OpenH264Decoder::new()
            .map_err(|e| MediaError::CodecError {
                details: format!("Failed to create OpenH264 decoder: {:?}", e),
            })?;

        Ok(Self {
            decoder,
            frame_count: 0,
        })
    }

    // Helper function removed - inlined into decode() method
}

impl VideoDecoder for H264Decoder {
    fn decode(&mut self, packet: &VideoPacket) -> Result<VideoFrame, MediaError> {
        if packet.data.is_empty() {
            return Err(MediaError::CodecError {
                details: "Empty packet data".to_string(),
            });
        }

        // Store packet metadata
        let is_keyframe = packet.is_keyframe;
        let dts = packet.dts;
        let pts = packet.pts;

        // Decode the H.264 packet
        let yuv_opt = self.decoder
            .decode(&packet.data)
            .map_err(|e| MediaError::CodecError {
                details: format!("H.264 decode error: {:?}", e),
            })?;

        // Check if we got a frame
        match yuv_opt {
            Some(yuv_frame) => {
                // Get dimensions using dimensions() method from YUVSource trait
                let (width, height) = yuv_frame.dimensions();

                // Get YUV planes
                let y_plane = yuv_frame.y();
                let u_plane = yuv_frame.u();
                let v_plane = yuv_frame.v();

                // For YUV420, calculate sizes
                let y_size = width * height;
                let uv_size = width * height / 4;

                // Create frame data
                let mut data = Vec::with_capacity(y_size + uv_size * 2);
                data.extend_from_slice(&y_plane[..y_size]);
                data.extend_from_slice(&u_plane[..uv_size]);
                data.extend_from_slice(&v_plane[..uv_size]);

                // Calculate timestamp
                let timestamp = if let Some(pts_value) = pts {
                    Duration::from_millis(pts_value as u64)
                } else {
                    Duration::from_millis(self.frame_count * 33)
                };

                // Increment frame count
                self.frame_count += 1;

                // Create and return frame
                Ok(VideoFrame {
                    width: width as u32,
                    height: height as u32,
                    format: PixelFormat::YUV420,
                    data,
                    timestamp,
                    duration: Some(Duration::from_millis(33)),
                    metadata: FrameMetadata {
                        is_keyframe,
                        pts,
                        dts,
                        sequence: Some(self.frame_count - 1),
                    },
                })
            }
            None => {
                // No frame decoded (buffering or waiting for keyframe)
                Err(MediaError::CodecError {
                    details: "No frame decoded (buffering)".to_string(),
                })
            }
        }
    }

    fn flush(&mut self) -> Result<Vec<VideoFrame>, MediaError> {
        // OpenH264 doesn't require explicit flushing
        // Return empty vec as there are no buffered frames
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_creation() {
        let result = H264Decoder::new();
        assert!(result.is_ok(), "Should create H.264 decoder");
    }

    #[test]
    fn test_empty_packet_error() {
        let mut decoder = H264Decoder::new().unwrap();
        let packet = VideoPacket {
            data: vec![],
            pts: None,
            dts: None,
            is_keyframe: false,
        };

        let result = decoder.decode(&packet);
        assert!(result.is_err(), "Empty packet should return error");
    }
}
