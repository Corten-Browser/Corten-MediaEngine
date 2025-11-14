//! # video_decoders Component
//!
//! Video codec implementations (H.264, VP9, AV1)
//!
//! This component provides decoder implementations for common video codecs
//! used in web browsers and media applications.
//!
//! # Examples
//!
//! ```no_run
//! use cortenbrowser_video_decoders::{DecoderFactory, H264Decoder};
//! use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level, VideoPacket};
//!
//! // Create a decoder using the factory
//! let codec = VideoCodec::H264 {
//!     profile: H264Profile::High,
//!     level: H264Level::Level4_1,
//!     hardware_accel: false,
//! };
//! let mut decoder = DecoderFactory::create_decoder(codec).unwrap();
//!
//! // Decode a packet
//! let packet = VideoPacket::default();
//! let frame = decoder.decode(&packet).unwrap();
//! ```

#![warn(missing_docs)]
// Note: unsafe code is required for FFI bindings to codec libraries
#![allow(unsafe_code)]

// Re-export shared types for convenience
pub use cortenbrowser_shared_types::{
    MediaError, VideoCodec, VideoDecoder, VideoFrame, VideoPacket,
};

// Conditional compilation based on features
#[cfg(feature = "h264")]
mod h264;

#[cfg(feature = "vp9")]
mod vp9;

#[cfg(feature = "av1")]
mod av1;

mod factory;

// Re-export public APIs conditionally
#[cfg(feature = "h264")]
pub use h264::H264Decoder;

#[cfg(feature = "vp9")]
pub use vp9::VP9Decoder;

#[cfg(feature = "av1")]
pub use av1::AV1Decoder;

pub use factory::DecoderFactory;
