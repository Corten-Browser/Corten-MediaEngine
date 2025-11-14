//! AAC audio decoder implementation
//!
//! Provides decoding of AAC-encoded audio packets to PCM samples.

use cortenbrowser_shared_types::{AudioBuffer, AudioDecoder, AudioFormat, AudioPacket, MediaError};
use std::io::Cursor;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// AAC audio decoder
///
/// Decodes AAC-encoded audio packets into PCM audio samples.
/// Uses Symphonia for pure Rust AAC decoding.
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_audio_decoders::AACDecoder;
/// use cortenbrowser_shared_types::{AudioDecoder, AudioPacket};
///
/// let mut decoder = AACDecoder::new().expect("Failed to create decoder");
/// let packet = AudioPacket {
///     data: vec![/* aac data */],
///     pts: Some(0),
///     dts: Some(0),
/// };
/// let buffer = decoder.decode(&packet).expect("Failed to decode");
/// ```
pub struct AACDecoder {
    // Symphonia decoders are created per-stream, so we'll initialize on first decode
    _initialized: bool,
}

impl AACDecoder {
    /// Create a new AAC decoder
    ///
    /// # Returns
    ///
    /// `Ok(AACDecoder)` on success
    pub fn new() -> Result<Self, MediaError> {
        Ok(Self {
            _initialized: false,
        })
    }

    /// Decode AAC packet using Symphonia
    fn decode_with_symphonia(&self, data: &[u8]) -> Result<(Vec<f32>, u32, u8), MediaError> {
        // Create a media source from the packet data - needs to own the data
        let owned_data = data.to_vec();
        let cursor = Cursor::new(owned_data);
        let media_source = MediaSourceStream::new(Box::new(cursor), Default::default());

        // Create a hint to help Symphonia identify the format
        let mut hint = Hint::new();
        hint.with_extension("aac");

        // Probe the media source
        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, media_source, &format_opts, &metadata_opts)
            .map_err(|e| MediaError::CodecError {
                details: format!("Failed to probe AAC format: {}", e),
            })?;

        let mut format = probed.format;

        // Get the default track
        let track = format
            .default_track()
            .ok_or_else(|| MediaError::CodecError {
                details: "No default track found in AAC stream".to_string(),
            })?;

        // Create decoder
        let decoder_opts = DecoderOptions::default();
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &decoder_opts)
            .map_err(|e| MediaError::CodecError {
                details: format!("Failed to create AAC decoder: {}", e),
            })?;

        // Decode the packet
        let packet = format.next_packet().map_err(|e| MediaError::CodecError {
            details: format!("Failed to read AAC packet: {}", e),
        })?;

        let decoded = decoder
            .decode(&packet)
            .map_err(|e| MediaError::CodecError {
                details: format!("Failed to decode AAC packet: {}", e),
            })?;

        // Extract audio data
        let (samples, sample_rate, channels) = match decoded {
            AudioBufferRef::F32(buf) => {
                let samples: Vec<f32> = buf.chan(0).to_vec();
                let spec = buf.spec();
                (samples, spec.rate, spec.channels.count() as u8)
            }
            AudioBufferRef::S16(buf) => {
                let samples: Vec<f32> = buf.chan(0).iter().map(|&s| s as f32 / 32768.0).collect();
                let spec = buf.spec();
                (samples, spec.rate, spec.channels.count() as u8)
            }
            _ => {
                return Err(MediaError::CodecError {
                    details: "Unsupported AAC sample format".to_string(),
                })
            }
        };

        Ok((samples, sample_rate, channels))
    }
}

impl AudioDecoder for AACDecoder {
    fn decode(&mut self, packet: &AudioPacket) -> Result<AudioBuffer, MediaError> {
        if packet.data.is_empty() {
            return Err(MediaError::CodecError {
                details: "Cannot decode empty packet".to_string(),
            });
        }

        // Attempt to decode with Symphonia
        let (samples, sample_rate, channels) = self.decode_with_symphonia(&packet.data)?;

        // Calculate timestamp
        let timestamp = if let Some(pts) = packet.pts {
            std::time::Duration::from_secs_f64(pts as f64 / sample_rate as f64)
        } else {
            std::time::Duration::ZERO
        };

        // Calculate duration
        let sample_count = samples.len() / channels as usize;
        let duration = std::time::Duration::from_secs_f64(sample_count as f64 / sample_rate as f64);

        Ok(AudioBuffer {
            format: AudioFormat::F32LE,
            sample_rate,
            channels,
            samples,
            timestamp,
            duration,
        })
    }

    fn flush(&mut self) -> Result<Vec<AudioBuffer>, MediaError> {
        // Symphonia handles buffering internally
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aac_decoder_creation() {
        let decoder = AACDecoder::new();
        assert!(decoder.is_ok());
    }
}
