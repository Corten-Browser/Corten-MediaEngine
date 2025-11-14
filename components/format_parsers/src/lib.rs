//! # format_parsers Component
//!
//! Container format demuxing and parsing (MP4, WebM, Ogg, Matroska)
//!
//! This crate provides parsers for common media container formats:
//! - **MP4**: MPEG-4 Part 14 container format
//! - **WebM**: WebM container based on Matroska
//! - **Ogg**: Ogg container for Vorbis, Opus, and Theora
//! - **Matroska (MKV)**: Matroska multimedia container
//!
//! # Examples
//!
//! ```no_run
//! use cortenbrowser_format_parsers::{Mp4Demuxer, Demuxer};
//!
//! let demuxer = Mp4Demuxer::new();
//! let data = std::fs::read("video.mp4").unwrap();
//! let info = demuxer.parse(&data).unwrap();
//!
//! println!("Duration: {:?}", info.duration);
//! println!("Video tracks: {}", info.video_tracks.len());
//! println!("Audio tracks: {}", info.audio_tracks.len());
//! ```

#![warn(missing_docs)]

mod demuxer;
mod matroska;
mod mp4;
mod ogg;
mod types;
mod webm;

// Re-export public API
pub use demuxer::Demuxer;
pub use matroska::MatroskaDemuxer;
pub use mp4::Mp4Demuxer;
pub use ogg::OggDemuxer;
pub use types::{AudioTrackInfo, MediaInfo, VideoTrackInfo};
pub use webm::WebmDemuxer;
