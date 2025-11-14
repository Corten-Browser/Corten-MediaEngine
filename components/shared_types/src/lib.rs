//! # shared_types Component
//!
//! Common types, enums, and traits used across all media components.
//!
//! This crate provides the foundational type definitions for the Corten Media Engine,
//! including codecs, formats, error types, media data structures, and core traits.
//!
//! # Overview
//!
//! The shared_types component is a foundational library that defines:
//!
//! - **Codec Types**: [`VideoCodec`], [`AudioCodec`] and their configuration
//! - **Formats**: [`PixelFormat`], [`AudioFormat`] for media data
//! - **Media Data**: [`VideoFrame`], [`AudioBuffer`], [`MediaSource`]
//! - **Errors**: [`MediaError`] for error handling
//! - **Sessions**: [`SessionId`] for session management
//! - **Traits**: [`MediaEngine`], [`Demuxer`], [`VideoDecoder`], [`AudioDecoder`]
//!
//! # Examples
//!
//! Creating a video codec specification:
//!
//! ```
//! use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};
//!
//! let codec = VideoCodec::H264 {
//!     profile: H264Profile::High,
//!     level: H264Level::Level4_1,
//!     hardware_accel: true,
//! };
//! ```
//!
//! Creating a video frame:
//!
//! ```
//! use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
//! use std::time::Duration;
//!
//! let frame = VideoFrame {
//!     width: 1920,
//!     height: 1080,
//!     format: PixelFormat::YUV420,
//!     data: vec![0u8; 1920 * 1080],
//!     timestamp: Duration::from_secs(1),
//!     duration: Some(Duration::from_millis(33)),
//!     metadata: FrameMetadata::default(),
//! };
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Module declarations
mod codecs;
mod errors;
mod formats;
mod media;
mod session;
mod traits;

// Re-export public API
pub use codecs::*;
pub use errors::*;
pub use formats::*;
pub use media::*;
pub use session::*;
pub use traits::*;
