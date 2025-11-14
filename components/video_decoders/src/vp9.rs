//! VP9 video decoder implementation
//!
//! This module provides VP9 decoding using the libvpx library (vpx-sys bindings).

use cortenbrowser_shared_types::{
    FrameMetadata, MediaError, PixelFormat, VideoDecoder, VideoFrame, VideoPacket,
};
use std::ptr;
use std::time::Duration;

/// VP9 video decoder
///
/// Decodes VP9 video packets into raw video frames using libvpx.
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_video_decoders::VP9Decoder;
/// use cortenbrowser_shared_types::{VideoDecoder, VideoPacket};
///
/// let mut decoder = VP9Decoder::new().unwrap();
/// let packet = VideoPacket::default();
/// let frame = decoder.decode(&packet).unwrap();
/// ```
pub struct VP9Decoder {
    /// VPX codec context
    ctx: Box<vpx_sys::vpx_codec_ctx_t>,
    /// Frame sequence counter
    frame_count: u64,
    /// Whether decoder is initialized
    initialized: bool,
}

impl VP9Decoder {
    /// Creates a new VP9 decoder instance
    ///
    /// # Errors
    ///
    /// Returns a `MediaError::CodecError` if decoder initialization fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cortenbrowser_video_decoders::VP9Decoder;
    ///
    /// let decoder = VP9Decoder::new().expect("Failed to create VP9 decoder");
    /// ```
    pub fn new() -> Result<Self, MediaError> {
        let mut ctx = Box::new(unsafe { std::mem::zeroed::<vpx_sys::vpx_codec_ctx_t>() });

        // Initialize VP9 decoder using libvpx
        let iface = unsafe { vpx_sys::vpx_codec_vp9_dx() };

        let ret = unsafe {
            vpx_sys::vpx_codec_dec_init_ver(
                ctx.as_mut(),
                iface,
                ptr::null(),
                0,
                vpx_sys::VPX_DECODER_ABI_VERSION as i32,
            )
        };

        if ret != vpx_sys::vpx_codec_err_t::VPX_CODEC_OK {
            return Err(MediaError::CodecError {
                details: format!("Failed to initialize VP9 decoder: error code {}", ret),
            });
        }

        Ok(Self {
            ctx,
            frame_count: 0,
            initialized: true,
        })
    }

    /// Converts VPX image to our VideoFrame format
    fn vpx_img_to_video_frame(
        &mut self,
        img: &vpx_sys::vpx_image_t,
        pts: Option<i64>,
    ) -> VideoFrame {
        let width = img.d_w;
        let height = img.d_h;

        // For YUV420, calculate total data size
        let y_size = (img.stride[0] as u32 * height) as usize;
        let u_size = (img.stride[1] as u32 * height / 2) as usize;
        let v_size = (img.stride[2] as u32 * height / 2) as usize;

        // Copy plane data
        let mut data = Vec::with_capacity(y_size + u_size + v_size);

        unsafe {
            let y_plane = std::slice::from_raw_parts(img.planes[0], y_size);
            let u_plane = std::slice::from_raw_parts(img.planes[1], u_size);
            let v_plane = std::slice::from_raw_parts(img.planes[2], v_size);

            data.extend_from_slice(y_plane);
            data.extend_from_slice(u_plane);
            data.extend_from_slice(v_plane);
        }

        let timestamp = if let Some(pts_value) = pts {
            Duration::from_millis(pts_value as u64)
        } else {
            Duration::from_millis(self.frame_count * 33)
        };

        self.frame_count += 1;

        VideoFrame {
            width,
            height,
            format: PixelFormat::YUV420,
            data,
            timestamp,
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata {
                is_keyframe: false,
                pts,
                dts: None,
                sequence: Some(self.frame_count - 1),
            },
        }
    }
}

impl VideoDecoder for VP9Decoder {
    fn decode(&mut self, packet: &VideoPacket) -> Result<VideoFrame, MediaError> {
        if !self.initialized {
            return Err(MediaError::CodecError {
                details: "Decoder not initialized".to_string(),
            });
        }

        if packet.data.is_empty() {
            return Err(MediaError::CodecError {
                details: "Empty packet data".to_string(),
            });
        }

        // Decode the VP9 packet
        let ret = unsafe {
            vpx_sys::vpx_codec_decode(
                self.ctx.as_mut(),
                packet.data.as_ptr(),
                packet.data.len() as u32,
                ptr::null_mut(),
                0,
            )
        };

        if ret != vpx_sys::vpx_codec_err_t::VPX_CODEC_OK {
            return Err(MediaError::CodecError {
                details: format!("VP9 decode error: {:?}", ret),
            });
        }

        // Get decoded frame
        let mut iter = ptr::null();
        let img = unsafe {
            vpx_sys::vpx_codec_get_frame(self.ctx.as_mut(), &mut iter)
        };

        if img.is_null() {
            return Err(MediaError::CodecError {
                details: "No frame decoded (buffering)".to_string(),
            });
        }

        let img_ref = unsafe { &*img };
        let mut frame = self.vpx_img_to_video_frame(img_ref, packet.pts);
        frame.metadata.is_keyframe = packet.is_keyframe;
        frame.metadata.dts = packet.dts;

        Ok(frame)
    }

    fn flush(&mut self) -> Result<Vec<VideoFrame>, MediaError> {
        // VP9 decoder doesn't typically buffer frames
        Ok(Vec::new())
    }
}

impl Drop for VP9Decoder {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                vpx_sys::vpx_codec_destroy(self.ctx.as_mut());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_creation() {
        let result = VP9Decoder::new();
        assert!(result.is_ok(), "Should create VP9 decoder");
    }

    #[test]
    fn test_empty_packet_error() {
        let mut decoder = VP9Decoder::new().unwrap();
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
