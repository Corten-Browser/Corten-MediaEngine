//! Decoder factory for creating audio decoders
//!
//! Provides a factory pattern for creating appropriate decoders based on codec type.

use crate::{AACDecoder, MP3Decoder, OpusDecoder};
use cortenbrowser_shared_types::{AudioCodec, AudioDecoder, MediaError};

/// Factory for creating audio decoders
///
/// Provides a centralized way to create decoders for different audio codecs.
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_audio_decoders::DecoderFactory;
/// use cortenbrowser_shared_types::{AudioCodec, OpusApplication};
///
/// let codec = AudioCodec::Opus {
///     sample_rate: 48000,
///     channels: 2,
///     application: OpusApplication::Audio,
/// };
///
/// let decoder = DecoderFactory::create_decoder(codec)
///     .expect("Failed to create decoder");
/// ```
pub struct DecoderFactory;

impl DecoderFactory {
    /// Create a decoder for the specified codec
    ///
    /// # Arguments
    ///
    /// * `codec` - The audio codec specification
    ///
    /// # Returns
    ///
    /// `Ok(Box<dyn AudioDecoder>)` on success, or `Err(MediaError)` if:
    /// - The codec is not supported
    /// - Decoder creation fails
    ///
    /// # Supported Codecs
    ///
    /// - Opus
    /// - MP3
    /// - AAC
    ///
    /// # Unsupported Codecs
    ///
    /// - Vorbis (use VorbisDecoder separately)
    /// - FLAC (not yet implemented)
    /// - PCM (no decoding needed)
    pub fn create_decoder(codec: AudioCodec) -> Result<Box<dyn AudioDecoder>, MediaError> {
        match codec {
            AudioCodec::Opus {
                sample_rate,
                channels,
                ..
            } => {
                let decoder = OpusDecoder::new(sample_rate, channels)?;
                Ok(Box::new(decoder))
            }

            AudioCodec::MP3 { .. } => {
                let decoder = MP3Decoder::new()?;
                Ok(Box::new(decoder))
            }

            AudioCodec::AAC { .. } => {
                let decoder = AACDecoder::new()?;
                Ok(Box::new(decoder))
            }

            AudioCodec::Vorbis => Err(MediaError::UnsupportedFormat {
                format: "Vorbis codec not yet implemented in factory".to_string(),
            }),

            AudioCodec::FLAC => Err(MediaError::UnsupportedFormat {
                format: "FLAC codec not yet implemented".to_string(),
            }),

            AudioCodec::PCM { .. } => Err(MediaError::UnsupportedFormat {
                format: "PCM does not require decoding".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cortenbrowser_shared_types::{AACProfile, MP3Layer, OpusApplication, PCMFormat};

    #[test]
    fn test_factory_creates_opus_decoder() {
        let codec = AudioCodec::Opus {
            sample_rate: 48000,
            channels: 2,
            application: OpusApplication::Audio,
        };
        let result = DecoderFactory::create_decoder(codec);
        assert!(result.is_ok());
    }

    #[test]
    fn test_factory_creates_mp3_decoder() {
        let codec = AudioCodec::MP3 {
            layer: MP3Layer::Layer3,
            bitrate: 128000,
        };
        let result = DecoderFactory::create_decoder(codec);
        assert!(result.is_ok());
    }

    #[test]
    fn test_factory_creates_aac_decoder() {
        let codec = AudioCodec::AAC {
            profile: AACProfile::LC,
            sample_rate: 48000,
            channels: 2,
        };
        let result = DecoderFactory::create_decoder(codec);
        assert!(result.is_ok());
    }

    #[test]
    fn test_factory_rejects_vorbis() {
        let codec = AudioCodec::Vorbis;
        let result = DecoderFactory::create_decoder(codec);
        assert!(result.is_err());
    }

    #[test]
    fn test_factory_rejects_flac() {
        let codec = AudioCodec::FLAC;
        let result = DecoderFactory::create_decoder(codec);
        assert!(result.is_err());
    }

    #[test]
    fn test_factory_rejects_pcm() {
        let codec = AudioCodec::PCM {
            format: PCMFormat::F32LE,
            sample_rate: 48000,
            channels: 2,
        };
        let result = DecoderFactory::create_decoder(codec);
        assert!(result.is_err());
    }
}
