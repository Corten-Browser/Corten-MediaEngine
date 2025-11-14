//! MP4 container format demuxer

use crate::demuxer::Demuxer;
use crate::types::{AudioTrackInfo, MediaInfo, VideoTrackInfo};
use cortenbrowser_shared_types::{
    AACProfile, AudioCodec, H264Level, H264Profile, MediaError, VideoCodec,
};
use std::collections::HashMap;
use std::io::Cursor;
use std::time::Duration;

/// MP4 (MPEG-4 Part 14) container demuxer
///
/// Parses MP4 container format and extracts video/audio track information.
#[derive(Debug, Default)]
pub struct Mp4Demuxer {
    media_info: Option<MediaInfo>,
}

impl Demuxer for Mp4Demuxer {
    fn new() -> Self {
        Self { media_info: None }
    }

    fn parse(&self, data: &[u8]) -> Result<MediaInfo, MediaError> {
        if data.is_empty() {
            return Err(MediaError::UnsupportedFormat {
                format: "Empty data".to_string(),
            });
        }

        let cursor = Cursor::new(data);
        let mp4_file = mp4::Mp4Reader::read_header(cursor, data.len() as u64).map_err(|e| {
            MediaError::UnsupportedFormat {
                format: format!("Failed to parse MP4: {}", e),
            }
        })?;

        let duration = Duration::from_millis(mp4_file.duration().as_millis() as u64);

        let mut video_tracks = Vec::new();
        let mut audio_tracks = Vec::new();

        // Extract video and audio tracks
        for track_id in mp4_file.tracks().keys() {
            if let Some(track) = mp4_file.tracks().get(track_id) {
                match track.track_type() {
                    Ok(mp4::TrackType::Video) => {
                        if let Some(video_info) = extract_video_track_info(*track_id, track) {
                            video_tracks.push(video_info);
                        }
                    }
                    Ok(mp4::TrackType::Audio) => {
                        if let Some(audio_info) = extract_audio_track_info(*track_id, track) {
                            audio_tracks.push(audio_info);
                        }
                    }
                    _ => {}
                }
            }
        }

        let metadata = HashMap::new(); // MP4 metadata extraction can be added later

        Ok(MediaInfo {
            duration,
            video_tracks,
            audio_tracks,
            metadata,
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

/// Extract video track information from MP4 track
fn extract_video_track_info(track_id: u32, track: &mp4::Mp4Track) -> Option<VideoTrackInfo> {
    let codec = match track.media_type() {
        Ok(mp4::MediaType::H264) => VideoCodec::H264 {
            profile: H264Profile::High,
            level: H264Level::Level4_1,
            hardware_accel: false,
        },
        Ok(mp4::MediaType::H265) => VideoCodec::H265 {
            profile: cortenbrowser_shared_types::H265Profile::Main,
            tier: cortenbrowser_shared_types::H265Tier::Main,
            level: cortenbrowser_shared_types::H265Level::Level5_0,
        },
        Ok(mp4::MediaType::VP9) => VideoCodec::VP9 {
            profile: cortenbrowser_shared_types::VP9Profile::Profile0,
        },
        _ => return None,
    };

    Some(VideoTrackInfo {
        track_id,
        codec,
        width: track.width() as u32,
        height: track.height() as u32,
        frame_rate: track.frame_rate() as f32,
        bitrate: Some(track.bitrate()),
    })
}

/// Extract audio track information from MP4 track
fn extract_audio_track_info(track_id: u32, track: &mp4::Mp4Track) -> Option<AudioTrackInfo> {
    let codec = match track.media_type() {
        Ok(mp4::MediaType::AAC) => AudioCodec::AAC {
            profile: AACProfile::LC,
            sample_rate: 48000, // Default value, would need to parse codec config for actual value
            channels: 2,        // Default stereo
        },
        _ => return None,
    };

    Some(AudioTrackInfo {
        track_id,
        codec,
        sample_rate: 48000, // Default value
        channels: 2,        // Default stereo
        bitrate: Some(track.bitrate()),
    })
}
