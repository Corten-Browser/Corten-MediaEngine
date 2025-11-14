//! Media data structures
//!
//! This module provides data structures for representing video frames,
//! audio buffers, and media sources.

use crate::formats::{AudioFormat, PixelFormat};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

/// Metadata associated with a video frame
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrameMetadata {
    /// Whether this is a keyframe
    pub is_keyframe: bool,
    /// Presentation timestamp (PTS)
    pub pts: Option<i64>,
    /// Decode timestamp (DTS)
    pub dts: Option<i64>,
    /// Frame sequence number
    pub sequence: Option<u64>,
}

/// Decoded video frame data
///
/// # Examples
///
/// ```
/// use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
/// use std::time::Duration;
///
/// let frame = VideoFrame {
///     width: 1920,
///     height: 1080,
///     format: PixelFormat::YUV420,
///     data: vec![0u8; 1920 * 1080],
///     timestamp: Duration::from_secs(1),
///     duration: Some(Duration::from_millis(33)),
///     metadata: FrameMetadata::default(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct VideoFrame {
    /// Frame width in pixels
    pub width: u32,
    /// Frame height in pixels
    pub height: u32,
    /// Pixel format
    pub format: PixelFormat,
    /// Raw pixel data
    pub data: Vec<u8>,
    /// Presentation timestamp
    pub timestamp: Duration,
    /// Frame duration (time until next frame)
    pub duration: Option<Duration>,
    /// Additional frame metadata
    pub metadata: FrameMetadata,
}

impl VideoFrame {
    /// Creates a new video frame
    pub fn new(
        width: u32,
        height: u32,
        format: PixelFormat,
        data: Vec<u8>,
        timestamp: Duration,
    ) -> Self {
        Self {
            width,
            height,
            format,
            data,
            timestamp,
            duration: None,
            metadata: FrameMetadata::default(),
        }
    }

    /// Returns the size of the frame data in bytes
    pub fn data_size(&self) -> usize {
        self.data.len()
    }
}

/// Decoded audio sample buffer
///
/// # Examples
///
/// ```
/// use cortenbrowser_shared_types::{AudioBuffer, AudioFormat};
/// use std::time::Duration;
///
/// let buffer = AudioBuffer {
///     format: AudioFormat::F32LE,
///     sample_rate: 48000,
///     channels: 2,
///     samples: vec![0.0f32; 4800],
///     timestamp: Duration::from_millis(100),
///     duration: Duration::from_millis(100),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AudioBuffer {
    /// Sample format
    pub format: AudioFormat,
    /// Sample rate in Hz
    pub sample_rate: u32,
    /// Number of channels
    pub channels: u8,
    /// Audio samples (interleaved if multi-channel)
    pub samples: Vec<f32>,
    /// Presentation timestamp
    pub timestamp: Duration,
    /// Buffer duration
    pub duration: Duration,
}

impl AudioBuffer {
    /// Creates a new audio buffer
    pub fn new(
        format: AudioFormat,
        sample_rate: u32,
        channels: u8,
        samples: Vec<f32>,
        timestamp: Duration,
    ) -> Self {
        let duration =
            Duration::from_secs_f64(samples.len() as f64 / (sample_rate as f64 * channels as f64));
        Self {
            format,
            sample_rate,
            channels,
            samples,
            timestamp,
            duration,
        }
    }

    /// Returns the number of samples per channel
    pub fn sample_count(&self) -> usize {
        self.samples.len() / self.channels as usize
    }
}

/// Media chunk for streaming
#[derive(Debug, Clone)]
pub struct MediaChunk {
    /// Chunk data
    pub data: Vec<u8>,
    /// Chunk sequence number
    pub sequence: u64,
    /// Whether this is the final chunk
    pub is_final: bool,
}

/// Source buffer for Media Source Extensions
#[derive(Debug, Clone)]
pub struct SourceBuffer {
    /// Buffer ID
    pub id: String,
    /// MIME type
    pub mime_type: String,
}

/// Placeholder for peer connection (WebRTC)
#[derive(Debug, Clone)]
pub struct PeerConnection {
    /// Connection ID
    pub id: String,
}

/// Capture device information
#[derive(Debug, Clone)]
pub struct CaptureDevice {
    /// Device ID
    pub id: String,
    /// Device name
    pub name: String,
    /// Device type
    pub device_type: CaptureDeviceType,
}

/// Type of capture device
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureDeviceType {
    /// Camera/webcam
    Camera,
    /// Microphone
    Microphone,
    /// Screen capture
    Screen,
}

/// Media constraints for capture
#[derive(Debug, Clone, Default)]
pub struct MediaConstraints {
    /// Video constraints
    pub video: Option<VideoConstraints>,
    /// Audio constraints
    pub audio: Option<AudioConstraints>,
}

/// Video capture constraints
#[derive(Debug, Clone)]
pub struct VideoConstraints {
    /// Desired width
    pub width: Option<u32>,
    /// Desired height
    pub height: Option<u32>,
    /// Desired frame rate
    pub frame_rate: Option<u32>,
}

/// Audio capture constraints
#[derive(Debug, Clone)]
pub struct AudioConstraints {
    /// Desired sample rate
    pub sample_rate: Option<u32>,
    /// Desired channel count
    pub channels: Option<u8>,
}

/// Source of media data
///
/// # Examples
///
/// ```
/// use cortenbrowser_shared_types::MediaSource;
///
/// let source = MediaSource::Url {
///     url: "https://example.com/video.mp4".to_string(),
/// };
/// ```
#[derive(Debug, Clone)]
pub enum MediaSource {
    /// URL to media file
    Url {
        /// The media URL
        url: String,
    },

    /// Raw bytes buffer
    Buffer {
        /// Media data
        data: Vec<u8>,
        /// MIME type
        mime_type: String,
    },

    /// Streaming source with chunks
    Stream {
        /// Receiver for media chunks
        #[allow(dead_code)]
        receiver: Arc<mpsc::Receiver<MediaChunk>>,
        /// MIME type
        mime_type: String,
    },

    /// Media Source Extensions
    MSE {
        /// Source buffers
        source_buffers: Vec<SourceBuffer>,
    },

    /// WebRTC media stream
    WebRTC {
        /// Peer connection
        peer_connection: Arc<PeerConnection>,
        /// Track ID
        track_id: String,
    },

    /// Device capture
    Capture {
        /// Capture device
        device: CaptureDevice,
        /// Media constraints
        constraints: MediaConstraints,
    },
}
