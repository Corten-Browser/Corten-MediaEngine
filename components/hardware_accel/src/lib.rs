//! # hardware_accel Component
//!
//! Hardware-accelerated video decoding for the Corten Media Engine.
//!
//! This component provides hardware video decoding support across multiple platforms:
//! - **Linux**: VA-API (Video Acceleration API)
//! - **Windows**: DXVA (DirectX Video Acceleration) - stub
//! - **macOS**: VideoToolbox - stub
//!
//! # Platform Support
//!
//! | Platform | API | Status | Codecs |
//! |----------|-----|--------|--------|
//! | Linux | VA-API | ✅ Implemented (mock) | H.264, VP9, VP8, H.265, AV1 |
//! | Windows | DXVA | ⚠️ Stub | N/A |
//! | macOS | VideoToolbox | ⚠️ Stub | N/A |
//!
//! # Architecture
//!
//! The component is organized around a platform-agnostic [`HardwareContext`] that:
//! - Detects available hardware acceleration
//! - Reports hardware capabilities
//! - Creates platform-specific decoders
//! - Provides automatic fallback when hardware is unavailable
//!
//! # Usage
//!
//! ## Basic Example
//!
//! ```no_run
//! use cortenbrowser_hardware_accel::HardwareContext;
//! use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level, VideoDecoder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create hardware context
//! let ctx = HardwareContext::new()?;
//!
//! // Check capabilities
//! let caps = ctx.get_capabilities();
//! println!("Max resolution: {}x{}", caps.max_resolution.0, caps.max_resolution.1);
//! println!("Supported codecs: {}", caps.supported_codecs.len());
//!
//! // Create decoder for H.264
//! let h264 = VideoCodec::H264 {
//!     profile: H264Profile::High,
//!     level: H264Level::Level4_1,
//!     hardware_accel: true,
//! };
//!
//! if ctx.is_codec_supported(&h264) {
//!     let mut decoder = ctx.create_decoder(&h264)?;
//!     // Use decoder for video playback...
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Codec Support Detection
//!
//! ```no_run
//! use cortenbrowser_hardware_accel::HardwareContext;
//! use cortenbrowser_shared_types::{VideoCodec, VP9Profile};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let ctx = HardwareContext::new()?;
//!
//! let vp9 = VideoCodec::VP9 {
//!     profile: VP9Profile::Profile0,
//! };
//!
//! if ctx.is_codec_supported(&vp9) {
//!     println!("VP9 hardware decoding available");
//! } else {
//!     println!("VP9 requires software fallback");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Graceful Degradation
//!
//! ```no_run
//! use cortenbrowser_hardware_accel::{HardwareContext, HardwareError};
//! use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};
//!
//! # fn main() {
//! let codec = VideoCodec::H264 {
//!     profile: H264Profile::High,
//!     level: H264Level::Level4_1,
//!     hardware_accel: true,
//! };
//!
//! match HardwareContext::new() {
//!     Ok(ctx) => {
//!         match ctx.create_decoder(&codec) {
//!             Ok(decoder) => {
//!                 println!("Using hardware decoder");
//!                 // Use hardware decoder
//!             }
//!             Err(e) => {
//!                 println!("Hardware unavailable: {}, falling back to software", e);
//!                 // Create software decoder instead
//!             }
//!         }
//!     }
//!     Err(HardwareError::NotAvailable) => {
//!         println!("Hardware acceleration not available, using software decoder");
//!         // Create software decoder
//!     }
//!     Err(e) => {
//!         eprintln!("Hardware error: {}", e);
//!     }
//! }
//! # }
//! ```
//!
//! # Error Handling
//!
//! All operations that can fail return [`HardwareResult<T>`](error::HardwareResult),
//! which is a type alias for `Result<T, HardwareError>`.
//!
//! Common error scenarios:
//! - [`HardwareError::NotAvailable`] - Hardware acceleration not available
//! - [`HardwareError::UnsupportedCodec`] - Requested codec not supported by hardware
//! - [`HardwareError::InitializationFailed`] - Hardware decoder initialization failed
//! - [`HardwareError::DecodeFailed`] - Hardware decode operation failed
//!
//! # Performance Considerations
//!
//! Hardware decoding provides significant benefits:
//! - **Lower CPU usage**: Typical 5-15% CPU vs 30-60% for software
//! - **Lower power consumption**: Up to 50% reduction in battery drain
//! - **Higher resolution support**: 4K@60fps with <10% CPU
//! - **Better thermal management**: Less heat generation
//!
//! # Platform-Specific Notes
//!
//! ## Linux (VA-API)
//!
//! Requires VA-API drivers:
//! ```bash
//! # Intel
//! sudo apt install intel-media-driver i965-va-driver
//!
//! # AMD
//! sudo apt install mesa-va-drivers
//!
//! # NVIDIA (with open-source drivers)
//! sudo apt install nvidia-vaapi-driver
//! ```
//!
//! Check VA-API support:
//! ```bash
//! vainfo
//! ```
//!
//! ## Windows (DXVA)
//!
//! **Status**: Stub implementation
//!
//! When implemented, will require:
//! - Windows Vista or later
//! - DirectX 11 or later
//! - GPU with DXVA2 support
//!
//! ## macOS (VideoToolbox)
//!
//! **Status**: Stub implementation
//!
//! When implemented, will require:
//! - macOS 10.8 or later
//! - Hardware encoder/decoder support (Intel Quick Sync or Apple Silicon)

#![warn(missing_docs)]

// Module declarations
mod capabilities;
mod context;
mod error;

#[cfg(target_os = "linux")]
mod vaapi;

#[cfg(target_os = "windows")]
mod dxva;

#[cfg(target_os = "macos")]
mod videotoolbox;

// Re-export public API
pub use capabilities::HardwareCapabilities;
pub use context::HardwareContext;
pub use error::{HardwareError, HardwareResult};

#[cfg(target_os = "linux")]
pub use vaapi::VAAPIDecoder;

#[cfg(target_os = "windows")]
pub use dxva::DXVADecoder;

#[cfg(target_os = "macos")]
pub use videotoolbox::VideoToolboxDecoder;
