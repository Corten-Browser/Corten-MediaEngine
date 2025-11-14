//! # media_capture Component
//!
//! Device capture APIs (screen capture, camera, microphone, MediaStream API)
//!
//! This component provides interfaces for capturing video and audio from
//! various sources including screens, cameras, and microphones.
//!
//! # Examples
//!
//! ```no_run
//! use cortenbrowser_media_capture::{DeviceEnumerator, CaptureConstraints};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let enumerator = DeviceEnumerator::new();
//!     let devices = enumerator.enumerate_video_devices().await?;
//!
//!     for device in devices {
//!         println!("Found device: {} ({})", device.label, device.device_id);
//!     }
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

mod types;
mod device_enumerator;
mod screen_capture;
mod camera_capture;
mod microphone_capture;

// Re-export public API
pub use types::*;
pub use device_enumerator::DeviceEnumerator;
pub use screen_capture::ScreenCapture;
pub use camera_capture::CameraCapture;
pub use microphone_capture::MicrophoneCapture;
