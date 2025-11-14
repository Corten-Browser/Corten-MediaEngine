//! Demuxer trait and related types

use crate::types::{AudioTrackInfo, MediaInfo, VideoTrackInfo};
use cortenbrowser_shared_types::MediaError;

/// Trait for container format demuxers
///
/// Demuxers are responsible for parsing container formats (MP4, WebM, etc.)
/// and extracting metadata about the contained media streams.
pub trait Demuxer {
    /// Create a new demuxer instance
    fn new() -> Self
    where
        Self: Sized;

    /// Parse media container data and extract information
    ///
    /// # Arguments
    ///
    /// * `data` - Raw container data to parse
    ///
    /// # Returns
    ///
    /// * `Ok(MediaInfo)` - Successfully parsed media information
    /// * `Err(MediaError)` - Failed to parse container
    fn parse(&self, data: &[u8]) -> Result<MediaInfo, MediaError>;

    /// Get information about a specific video track
    ///
    /// # Arguments
    ///
    /// * `track_id` - Identifier of the track to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(VideoTrackInfo)` - Track information if found
    /// * `None` - Track not found
    fn get_video_track(&self, track_id: u32) -> Option<VideoTrackInfo>;

    /// Get information about a specific audio track
    ///
    /// # Arguments
    ///
    /// * `track_id` - Identifier of the track to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(AudioTrackInfo)` - Track information if found
    /// * `None` - Track not found
    fn get_audio_track(&self, track_id: u32) -> Option<AudioTrackInfo>;
}
