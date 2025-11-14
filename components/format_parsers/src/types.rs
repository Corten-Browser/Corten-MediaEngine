//! Type definitions for media information and track metadata

use cortenbrowser_shared_types::{AudioCodec, VideoCodec};
use std::collections::HashMap;
use std::time::Duration;

/// Information about a media container
#[derive(Debug, Clone, PartialEq)]
pub struct MediaInfo {
    /// Total duration of the media
    pub duration: Duration,
    /// Video tracks in the container
    pub video_tracks: Vec<VideoTrackInfo>,
    /// Audio tracks in the container
    pub audio_tracks: Vec<AudioTrackInfo>,
    /// Container metadata (title, author, etc.)
    pub metadata: HashMap<String, String>,
}

/// Information about a video track
#[derive(Debug, Clone, PartialEq)]
pub struct VideoTrackInfo {
    /// Track identifier
    pub track_id: u32,
    /// Video codec used
    pub codec: VideoCodec,
    /// Video width in pixels
    pub width: u32,
    /// Video height in pixels
    pub height: u32,
    /// Frame rate in frames per second
    pub frame_rate: f32,
    /// Bitrate in bits per second (if available)
    pub bitrate: Option<u32>,
}

/// Information about an audio track
#[derive(Debug, Clone, PartialEq)]
pub struct AudioTrackInfo {
    /// Track identifier
    pub track_id: u32,
    /// Audio codec used
    pub codec: AudioCodec,
    /// Sample rate in Hz
    pub sample_rate: u32,
    /// Number of audio channels
    pub channels: u8,
    /// Bitrate in bits per second (if available)
    pub bitrate: Option<u32>,
}

impl Default for MediaInfo {
    fn default() -> Self {
        Self {
            duration: Duration::ZERO,
            video_tracks: Vec::new(),
            audio_tracks: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}
