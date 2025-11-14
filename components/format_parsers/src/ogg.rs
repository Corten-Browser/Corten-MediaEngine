//! Ogg container format demuxer

use crate::demuxer::Demuxer;
use crate::types::{AudioTrackInfo, MediaInfo, VideoTrackInfo};
use cortenbrowser_shared_types::{AudioCodec, MediaError};
use std::collections::HashMap;
use std::io::Cursor;
use std::time::Duration;

/// Ogg container demuxer
///
/// Parses Ogg container format and extracts media information.
#[derive(Debug, Default)]
pub struct OggDemuxer {
    media_info: Option<MediaInfo>,
}

impl Demuxer for OggDemuxer {
    fn new() -> Self {
        Self { media_info: None }
    }

    fn parse(&self, data: &[u8]) -> Result<MediaInfo, MediaError> {
        if data.is_empty() {
            return Err(MediaError::UnsupportedFormat {
                format: "Empty data".to_string(),
            });
        }

        // Basic Ogg validation - must start with "OggS"
        if data.len() < 4 || &data[0..4] != b"OggS" {
            return Err(MediaError::UnsupportedFormat {
                format: "Invalid Ogg data".to_string(),
            });
        }

        let cursor = Cursor::new(data);
        let mut reader = ogg::PacketReader::new(cursor);

        let mut audio_tracks = Vec::new();

        // Read first packet to identify codec
        if let Ok(Some(packet)) = reader.read_packet() {
            // Try to identify codec from packet header
            // Vorbis packets start with 0x01 + "vorbis"
            // Opus packets start with "OpusHead"
            if packet.data.len() > 8 {
                if &packet.data[1..7] == b"vorbis" {
                    audio_tracks.push(AudioTrackInfo {
                        track_id: packet.stream_serial(),
                        codec: AudioCodec::Vorbis,
                        sample_rate: 44100, // Default, would parse from header
                        channels: 2,
                        bitrate: None,
                    });
                } else if packet.data.starts_with(b"OpusHead") {
                    audio_tracks.push(AudioTrackInfo {
                        track_id: packet.stream_serial(),
                        codec: AudioCodec::Opus {
                            sample_rate: 48000, // Opus is always 48kHz internally
                            channels: 2,
                            application: cortenbrowser_shared_types::OpusApplication::Audio,
                        },
                        sample_rate: 48000,
                        channels: 2,
                        bitrate: None,
                    });
                }
            }
        }

        Ok(MediaInfo {
            duration: Duration::ZERO, // Would need to scan file for duration
            video_tracks: Vec::new(), // Ogg can contain Theora but not common
            audio_tracks,
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
