//! Opus audio decoder implementation
//!
//! Provides decoding of Opus-encoded audio packets to PCM samples.

use cortenbrowser_shared_types::{AudioBuffer, AudioDecoder, AudioFormat, AudioPacket, MediaError};
use opus::{Channels, Decoder};

/// Opus audio decoder
///
/// Decodes Opus-encoded audio packets into PCM audio samples.
/// Opus supports sample rates of 8000, 12000, 16000, 24000, and 48000 Hz.
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_audio_decoders::OpusDecoder;
/// use cortenbrowser_shared_types::{AudioDecoder, AudioPacket};
///
/// let mut decoder = OpusDecoder::new(48000, 2).expect("Failed to create decoder");
/// let packet = AudioPacket {
///     data: vec![/* opus data */],
///     pts: Some(0),
///     dts: Some(0),
/// };
/// let buffer = decoder.decode(&packet).expect("Failed to decode");
/// ```
pub struct OpusDecoder {
    decoder: Decoder,
    sample_rate: u32,
    channels: u8,
}

impl OpusDecoder {
    /// Create a new Opus decoder
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - Sample rate in Hz (must be 8000, 12000, 16000, 24000, or 48000)
    /// * `channels` - Number of channels (1 for mono, 2 for stereo)
    ///
    /// # Returns
    ///
    /// `Ok(OpusDecoder)` on success, or `Err(MediaError)` if parameters are invalid
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Sample rate is not one of the supported values
    /// - Channel count is invalid (0 or > 2)
    /// - Opus decoder creation fails
    pub fn new(sample_rate: u32, channels: u8) -> Result<Self, MediaError> {
        // Validate sample rate
        if ![8000, 12000, 16000, 24000, 48000].contains(&sample_rate) {
            return Err(MediaError::CodecError {
                details: format!(
                    "Opus decoder requires sample rate of 8000, 12000, 16000, 24000, or 48000 Hz, got {}",
                    sample_rate
                ),
            });
        }

        // Validate channels
        if channels == 0 || channels > 2 {
            return Err(MediaError::CodecError {
                details: format!("Opus decoder supports 1 or 2 channels, got {}", channels),
            });
        }

        // Create Opus decoder
        let opus_channels = if channels == 1 {
            Channels::Mono
        } else {
            Channels::Stereo
        };

        let decoder =
            Decoder::new(sample_rate, opus_channels).map_err(|e| MediaError::CodecError {
                details: format!("Failed to create Opus decoder: {}", e),
            })?;

        Ok(Self {
            decoder,
            sample_rate,
            channels,
        })
    }
}

impl AudioDecoder for OpusDecoder {
    fn decode(&mut self, packet: &AudioPacket) -> Result<AudioBuffer, MediaError> {
        if packet.data.is_empty() {
            return Err(MediaError::CodecError {
                details: "Cannot decode empty packet".to_string(),
            });
        }

        // Opus frames are typically 2.5, 5, 10, 20, 40, or 60 ms
        // For 48kHz, 20ms = 960 samples per channel
        // Maximum frame size for Opus is 120ms at 48kHz = 5760 samples
        let max_frame_size = 5760;
        let mut output = vec![0f32; max_frame_size * self.channels as usize];

        // Decode the Opus packet
        let samples_decoded = self
            .decoder
            .decode_float(&packet.data, &mut output, false)
            .map_err(|e| MediaError::CodecError {
                details: format!("Opus decoding failed: {}", e),
            })?;

        // Truncate to actual decoded size
        output.truncate(samples_decoded * self.channels as usize);

        // Calculate timestamp and duration
        let timestamp = if let Some(pts) = packet.pts {
            std::time::Duration::from_secs_f64(pts as f64 / self.sample_rate as f64)
        } else {
            std::time::Duration::ZERO
        };

        Ok(AudioBuffer {
            format: AudioFormat::F32LE,
            sample_rate: self.sample_rate,
            channels: self.channels,
            samples: output,
            timestamp,
            duration: std::time::Duration::from_secs_f64(
                samples_decoded as f64 / self.sample_rate as f64,
            ),
        })
    }

    fn flush(&mut self) -> Result<Vec<AudioBuffer>, MediaError> {
        // Opus decoder doesn't buffer frames, so nothing to flush
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opus_decoder_creation() {
        let decoder = OpusDecoder::new(48000, 2);
        assert!(decoder.is_ok());
    }

    #[test]
    fn test_opus_decoder_invalid_sample_rate() {
        let decoder = OpusDecoder::new(44100, 2);
        assert!(decoder.is_err());
    }

    #[test]
    fn test_opus_decoder_invalid_channels() {
        let decoder = OpusDecoder::new(48000, 0);
        assert!(decoder.is_err());
    }
}
