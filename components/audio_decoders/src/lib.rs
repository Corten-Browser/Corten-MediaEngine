//! # audio_decoders Component
//!
//! Audio codec implementations (AAC, MP3, Opus, Vorbis, PCM)
//!
//! This component provides decoder implementations for common audio codecs
//! used in web media playback. Each decoder implements the `AudioDecoder` trait
//! from `shared_types` and can be created via the `DecoderFactory`.
//!
//! # Examples
//!
//! ```no_run
//! use cortenbrowser_audio_decoders::{OpusDecoder, DecoderFactory};
//! use cortenbrowser_shared_types::{AudioCodec, AudioDecoder, OpusApplication};
//!
//! // Create Opus decoder directly
//! let opus_decoder = OpusDecoder::new(48000, 2).expect("Failed to create decoder");
//!
//! // Or use factory
//! let codec = AudioCodec::Opus {
//!     sample_rate: 48000,
//!     channels: 2,
//!     application: OpusApplication::Audio,
//! };
//! let decoder = DecoderFactory::create_decoder(codec).expect("Failed to create decoder");
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Re-export shared types for convenience
pub use cortenbrowser_shared_types::{
    AudioBuffer, AudioCodec, AudioDecoder, AudioPacket, MediaError,
};

// Module declarations
mod aac_decoder;
mod factory;
mod mp3_decoder;
mod opus_decoder;

// Re-export decoder implementations
pub use aac_decoder::AACDecoder;
pub use factory::DecoderFactory;
pub use mp3_decoder::MP3Decoder;
pub use opus_decoder::OpusDecoder;
