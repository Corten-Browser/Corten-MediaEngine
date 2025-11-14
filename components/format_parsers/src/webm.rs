//! WebM container format demuxer

use crate::demuxer::Demuxer;
use crate::types::{AudioTrackInfo, MediaInfo, VideoTrackInfo};
use cortenbrowser_shared_types::MediaError;
use std::collections::HashMap;
use std::time::Duration;

/// WebM container demuxer
///
/// Parses WebM container format (based on Matroska) and extracts media information.
#[derive(Debug, Default)]
pub struct WebmDemuxer {
    media_info: Option<MediaInfo>,
}

impl Demuxer for WebmDemuxer {
    fn new() -> Self {
        Self { media_info: None }
    }

    fn parse(&self, data: &[u8]) -> Result<MediaInfo, MediaError> {
        if data.is_empty() {
            return Err(MediaError::UnsupportedFormat {
                format: "Empty data".to_string(),
            });
        }

        // Basic WebM validation - check for EBML header
        if data.len() < 4 || &data[0..4] != b"\x1a\x45\xdf\xa3" {
            return Err(MediaError::UnsupportedFormat {
                format: "Invalid WebM data".to_string(),
            });
        }

        // Simplified implementation - returns valid but minimal MediaInfo
        Ok(MediaInfo {
            duration: Duration::ZERO,
            video_tracks: Vec::new(),
            audio_tracks: Vec::new(),
            metadata: HashMap::new(),
        })
    }

    fn get_video_track(&self, track_id: u32) -> Option<VideoTrackInfo> {
        self.media_info
            .as_ref()?
            .video_tracks
            .iter()
            .find(|t| t.track_id == track_id)
            .cloned()
    }

    fn get_audio_track(&self, track_id: u32) -> Option<AudioTrackInfo> {
        self.media_info
            .as_ref()?
            .audio_tracks
            .iter()
            .find(|t| t.track_id == track_id)
            .cloned()
    }
}
