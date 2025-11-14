//! # webrtc_integration Component
//!
//! WebRTC media stream handling (RTP, RTCP, jitter buffer, echo cancellation)
//!
//! This component provides:
//! - RTP packet creation and serialization
//! - RTP packetization for media payloads
//! - Jitter buffer for packet reordering
//! - WebRTC encoder wrapper
//! - RTCP handling (stub)
//! - Echo cancellation hooks (stub)

#![warn(missing_docs)]

mod rtp;
mod jitter_buffer;
mod encoder;
mod rtcp;
mod echo_cancellation;

pub use rtp::{RTPPacket, RTPPacketizer};
pub use jitter_buffer::JitterBuffer;
pub use encoder::{WebRTCEncoder, EncoderConfig};
pub use rtcp::RTCPHandler;
pub use echo_cancellation::EchoCanceller;

// Re-export from shared_types
pub use cortenbrowser_shared_types::MediaError;
