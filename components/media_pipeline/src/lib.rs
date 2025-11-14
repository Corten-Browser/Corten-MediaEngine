//! # media_pipeline Component
//!
//! Pipeline orchestration and A/V synchronization (source readers, demuxer/decoder coordination)
//!
//! This component provides the core media pipeline infrastructure for coordinating
//! source readers, demuxers, decoders, and audio/video synchronization.
//!
//! # Overview
//!
//! The media_pipeline component consists of:
//!
//! - [`AVSyncController`]: Audio/video synchronization logic
//! - [`MediaPipeline`]: Main pipeline orchestration (coming soon)
//! - [`PipelineConfig`]: Pipeline configuration
//! - [`SyncDecision`]: Synchronization decisions
//!
//! # Examples
//!
//! ```
//! use cortenbrowser_media_pipeline::{AVSyncController, SyncDecision};
//! use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
//! use std::time::Duration;
//!
//! let controller = AVSyncController::new();
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
//!
//! let decision = controller.sync_frame(&frame, Duration::from_secs(1));
//! assert_eq!(decision, SyncDecision::Display);
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod pipeline;
mod sync;
mod types;

// Re-export public API
pub use pipeline::MediaPipeline;
pub use sync::AVSyncController;
pub use types::{PipelineConfig, SyncDecision};
