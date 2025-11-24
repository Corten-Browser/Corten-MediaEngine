//! AV1 video decoder implementation
//!
//! This module provides AV1 decoding using the dav1d library.

use cortenbrowser_shared_types::{
    FrameMetadata, MediaError, PixelFormat, VideoDecoder, VideoFrame, VideoPacket,
};
use dav1d::{Decoder as Dav1dDecoder, PixelLayout, PlanarImageComponent};
use std::time::Duration;

/// AV1 video decoder
///
/// Decodes AV1 video packets into raw video frames using dav1d.
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_video_decoders::AV1Decoder;
/// use cortenbrowser_shared_types::{VideoDecoder, VideoPacket};
///
/// let mut decoder = AV1Decoder::new().unwrap();
/// let packet = VideoPacket::default();
/// let frame = decoder.decode(&packet).unwrap();
/// ```
pub struct AV1Decoder {
    /// Underlying dav1d decoder instance
    decoder: Dav1dDecoder,
    /// Frame sequence counter
    frame_count: u64,
}

impl AV1Decoder {
    /// Creates a new AV1 decoder instance
    ///
    /// # Errors
    ///
    /// Returns a `MediaError::CodecError` if decoder initialization fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cortenbrowser_video_decoders::AV1Decoder;
    ///
    /// let decoder = AV1Decoder::new().expect("Failed to create AV1 decoder");
    /// ```
    pub fn new() -> Result<Self, MediaError> {
        let decoder = Dav1dDecoder::new()
            .map_err(|e| MediaError::CodecError {
                details: format!("Failed to create dav1d decoder: {:?}", e),
            })?;

        Ok(Self {
            decoder,
            frame_count: 0,
        })
    }

    /// Converts dav1d picture to our VideoFrame format
    fn picture_to_video_frame(
        &mut self,
        picture: &dav1d::Picture,
        pts: Option<i64>,
    ) -> Result<VideoFrame, MediaError> {
        let width = picture.width();
        let height = picture.height();

        // Get picture planes based on pixel layout
        let data = match picture.pixel_layout() {
            PixelLayout::I420 => {
                // YUV420 format
                let stride_y = picture.stride(PlanarImageComponent::Y) as usize;
                let stride_u = picture.stride(PlanarImageComponent::U) as usize;
                let stride_v = picture.stride(PlanarImageComponent::V) as usize;

                let plane_y = picture.plane(PlanarImageComponent::Y);
                let plane_u = picture.plane(PlanarImageComponent::U);
                let plane_v = picture.plane(PlanarImageComponent::V);

                let h = height as usize;
                let y_size = stride_y * h;
                let u_size = stride_u * (h / 2);
                let v_size = stride_v * (h / 2);

                let mut data = Vec::with_capacity(y_size + u_size + v_size);
                data.extend_from_slice(plane_y.as_ref());
                data.extend_from_slice(plane_u.as_ref());
                data.extend_from_slice(plane_v.as_ref());
                data
            }
            _ => {
                return Err(MediaError::CodecError {
                    details: "Unsupported pixel layout".to_string(),
                });
            }
        };

        let timestamp = if let Some(pts_value) = pts {
            Duration::from_millis(pts_value as u64)
        } else {
            Duration::from_millis(self.frame_count * 33)
        };

        self.frame_count += 1;

        Ok(VideoFrame {
            width: width as u32,
            height: height as u32,
            format: PixelFormat::YUV420,
            data,
            timestamp,
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata {
                is_keyframe: false, // Will be set from packet
                pts,
                dts: None,
                sequence: Some(self.frame_count - 1),
            },
        })
    }
}

impl VideoDecoder for AV1Decoder {
    fn decode(&mut self, packet: &VideoPacket) -> Result<VideoFrame, MediaError> {
        if packet.data.is_empty() {
            return Err(MediaError::CodecError {
                details: "Empty packet data".to_string(),
            });
        }

        // Send data to decoder
        self.decoder
            .send_data(packet.data.clone(), None, None, None)
            .map_err(|e| MediaError::CodecError {
                details: format!("AV1 send_data error: {:?}", e),
            })?;

        // Get decoded picture
        match self.decoder.get_picture() {
            Ok(picture) => {
                let mut frame = self.picture_to_video_frame(&picture, packet.pts)?;
                frame.metadata.is_keyframe = packet.is_keyframe;
                frame.metadata.dts = packet.dts;
                Ok(frame)
            }
            Err(e) => Err(MediaError::CodecError {
                details: format!("AV1 get_picture error: {:?}", e),
            }),
        }
    }

    fn flush(&mut self) -> Result<Vec<VideoFrame>, MediaError> {
        let mut frames = Vec::new();

        // Flush the decoder
        self.decoder.flush();

        // Get any remaining frames
        while let Ok(picture) = self.decoder.get_picture() {
            if let Ok(frame) = self.picture_to_video_frame(&picture, None) {
                frames.push(frame);
            }
        }

        Ok(frames)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_creation() {
        let result = AV1Decoder::new();
        assert!(result.is_ok(), "Should create AV1 decoder");
    }

    #[test]
    fn test_empty_packet_error() {
        let mut decoder = AV1Decoder::new().unwrap();
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
