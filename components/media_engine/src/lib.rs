//! # media_engine Component
//!
//! Main public API and component coordination (MediaEngine trait, message bus integration, session orchestration)
//!
//! This component provides the main MediaEngine implementation that coordinates all other
//! media components including session management, pipeline orchestration, format parsing,
//! decoding, buffering, and synchronization.
//!
//! # Overview
//!
//! The media_engine component is the final integration layer that ties together:
//!
//! - **Session Management**: Using media_session for lifecycle and state
//! - **Pipeline Orchestration**: Using media_pipeline for playback coordination
//! - **Format Support**: Using format_parsers for container demuxing
//! - **Decoding**: Using video_decoders and audio_decoders for codec support
//! - **Buffering**: Using buffer_manager for data management
//! - **Hardware Acceleration**: Using hardware_accel for GPU decoding
//! - **WebRTC**: Using webrtc_integration for real-time media
//! - **DRM**: Using drm_support for protected content
//! - **Capture**: Using media_capture for device input
//!
//! # Examples
//!
//! ```no_run
//! use cortenbrowser_media_engine::{MediaEngineImpl, MediaEngineConfig};
//! use cortenbrowser_shared_types::{MediaEngine, MediaSessionConfig, MediaSource};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create media engine
//!     let config = MediaEngineConfig::default();
//!     let engine = MediaEngineImpl::new(config)?;
//!
//!     // Create a session
//!     let session_config = MediaSessionConfig::default();
//!     let session = engine.create_session(session_config).await?;
//!
//!     // Load media source
//!     let source = MediaSource::Url { url: "https://example.com/video.mp4".to_string() };
//!     engine.load_source(session, source).await?;
//!
//!     // Play
//!     engine.play(session).await?;
//!
//!     // Seek
//!     engine.seek(session, Duration::from_secs(30)).await?;
//!
//!     // Pause
//!     engine.pause(session).await?;
//!
//!     // Cleanup
//!     engine.destroy_session(session).await?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod engine;
mod types;

// Re-export public API
pub use engine::MediaEngineImpl;
pub use types::{MediaEngineConfig, MediaEngineEvent, MediaEngineMessage};
