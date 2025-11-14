//! Core trait definitions for media engine components
//!
//! This module defines the main interfaces that media engine components must implement.

use crate::codecs::{AudioCodec, VideoCodec};
use crate::errors::MediaError;
use crate::media::{AudioBuffer, MediaSource, VideoFrame};
use crate::session::{MediaSessionConfig, SessionId};
use std::time::Duration;
use tokio::sync::mpsc;

/// Time range for buffered media
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeRange {
    /// Start time
    pub start: Duration,
    /// End time
    pub end: Duration,
}

impl TimeRange {
    /// Creates a new time range
    pub fn new(start: Duration, end: Duration) -> Self {
        Self { start, end }
    }

    /// Returns the duration of this time range
    pub fn duration(&self) -> Duration {
        self.end.saturating_sub(self.start)
    }

    /// Checks if a time point is contained in this range
    pub fn contains(&self, time: Duration) -> bool {
        time >= self.start && time <= self.end
    }
}

/// Main media engine interface
///
/// This trait defines the core operations for media playback.
/// Implementations handle loading, playing, and controlling media.
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_shared_types::{MediaEngine, MediaSource, SessionId};
/// use std::time::Duration;
///
/// async fn play_video<E: MediaEngine>(engine: &E) -> Result<(), Box<dyn std::error::Error>> {
///     let session = engine.create_session(Default::default()).await?;
///     engine.load_source(session, MediaSource::Url {
///         url: "video.mp4".to_string()
///     }).await?;
///     engine.play(session).await?;
///     Ok(())
/// }
/// ```
#[allow(async_fn_in_trait)]
pub trait MediaEngine: Send + Sync {
    /// Initialize a new media session
    async fn create_session(&self, config: MediaSessionConfig) -> Result<SessionId, MediaError>;

    /// Load media from a source
    async fn load_source(&self, session: SessionId, source: MediaSource) -> Result<(), MediaError>;

    /// Start playback
    async fn play(&self, session: SessionId) -> Result<(), MediaError>;

    /// Pause playback
    async fn pause(&self, session: SessionId) -> Result<(), MediaError>;

    /// Seek to a specific position
    async fn seek(&self, session: SessionId, position: Duration) -> Result<(), MediaError>;

    /// Set playback volume (0.0 to 1.0)
    async fn set_volume(&self, session: SessionId, volume: f32) -> Result<(), MediaError>;

    /// Get the next video frame
    async fn get_video_frame(&self, session: SessionId) -> Result<VideoFrame, MediaError>;

    /// Get audio samples
    async fn get_audio_samples(
        &self,
        session: SessionId,
        count: usize,
    ) -> Result<AudioBuffer, MediaError>;

    /// Destroy a session and free resources
    async fn destroy_session(&self, session: SessionId) -> Result<(), MediaError>;
}

/// Media information from demuxer
#[derive(Debug, Clone, Default)]
pub struct MediaInfo {
    /// Duration of the media
    pub duration: Option<Duration>,
    /// Video tracks
    pub video_tracks: Vec<VideoTrackInfo>,
    /// Audio tracks
    pub audio_tracks: Vec<AudioTrackInfo>,
    /// Media title
    pub title: Option<String>,
}

/// Video track information
#[derive(Debug, Clone)]
pub struct VideoTrackInfo {
    /// Track ID
    pub id: u32,
    /// Codec
    pub codec: VideoCodec,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Frame rate
    pub frame_rate: Option<f64>,
}

/// Audio track information
#[derive(Debug, Clone)]
pub struct AudioTrackInfo {
    /// Track ID
    pub id: u32,
    /// Codec
    pub codec: AudioCodec,
    /// Sample rate
    pub sample_rate: u32,
    /// Number of channels
    pub channels: u8,
}

/// Video packet from demuxer
#[derive(Debug, Clone, Default)]
pub struct VideoPacket {
    /// Packet data
    pub data: Vec<u8>,
    /// Presentation timestamp
    pub pts: Option<i64>,
    /// Decode timestamp
    pub dts: Option<i64>,
    /// Whether this is a keyframe
    pub is_keyframe: bool,
}

/// Audio packet from demuxer
#[derive(Debug, Clone, Default)]
pub struct AudioPacket {
    /// Packet data
    pub data: Vec<u8>,
    /// Presentation timestamp
    pub pts: Option<i64>,
    /// Decode timestamp
    pub dts: Option<i64>,
}

/// Container format demuxer interface
///
/// Demuxers parse container formats (MP4, WebM, etc.) and extract
/// individual video and audio packets.
pub trait Demuxer {
    /// Parse media data and extract metadata
    fn parse(&self, data: &[u8]) -> Result<MediaInfo, MediaError>;

    /// Get receiver for video packets
    fn get_video_packets(&self) -> mpsc::Receiver<VideoPacket>;

    /// Get receiver for audio packets
    fn get_audio_packets(&self) -> mpsc::Receiver<AudioPacket>;
}

/// Video decoder interface
///
/// Decoders convert compressed video packets into raw video frames.
pub trait VideoDecoder {
    /// Decode a video packet into a frame
    fn decode(&mut self, packet: &VideoPacket) -> Result<VideoFrame, MediaError>;

    /// Flush any buffered frames
    fn flush(&mut self) -> Result<Vec<VideoFrame>, MediaError>;
}

/// Audio decoder interface
///
/// Decoders convert compressed audio packets into raw audio samples.
pub trait AudioDecoder {
    /// Decode an audio packet into samples
    fn decode(&mut self, packet: &AudioPacket) -> Result<AudioBuffer, MediaError>;

    /// Flush any buffered samples
    fn flush(&mut self) -> Result<Vec<AudioBuffer>, MediaError>;
}
