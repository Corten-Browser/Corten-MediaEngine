//! MP3 audio decoder implementation
//!
//! Provides decoding of MP3-encoded audio packets to PCM samples.

use cortenbrowser_shared_types::{AudioBuffer, AudioDecoder, AudioFormat, AudioPacket, MediaError};
use minimp3::Decoder;

/// MP3 audio decoder
///
/// Decodes MP3-encoded audio packets into PCM audio samples.
/// Supports all MP3 layers (I, II, III) and common sample rates.
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_audio_decoders::MP3Decoder;
/// use cortenbrowser_shared_types::{AudioDecoder, AudioPacket};
///
/// let mut decoder = MP3Decoder::new().expect("Failed to create decoder");
/// let packet = AudioPacket {
///     data: vec![/* mp3 data */],
///     pts: Some(0),
///     dts: Some(0),
/// };
/// let buffer = decoder.decode(&packet).expect("Failed to decode");
/// ```
pub struct MP3Decoder;

impl MP3Decoder {
    /// Create a new MP3 decoder
    ///
    /// # Returns
    ///
    /// `Ok(MP3Decoder)` on success
    pub fn new() -> Result<Self, MediaError> {
        Ok(Self)
    }
}

impl AudioDecoder for MP3Decoder {
    fn decode(&mut self, packet: &AudioPacket) -> Result<AudioBuffer, MediaError> {
        if packet.data.is_empty() {
            return Err(MediaError::CodecError {
                details: "Cannot decode empty packet".to_string(),
            });
        }

        // Create a new decoder with the packet data
        let cursor = std::io::Cursor::new(packet.data.clone());
        let mut temp_decoder = Decoder::new(cursor);

        // Decode the MP3 frame
        let frame = temp_decoder
            .next_frame()
            .map_err(|e| MediaError::CodecError {
                details: format!("MP3 decoding failed: {:?}", e),
            })?;

        // Convert i16 samples to f32
        let samples: Vec<f32> = frame.data.iter().map(|&s| s as f32 / 32768.0).collect();

        // Calculate timestamp
        let timestamp = if let Some(pts) = packet.pts {
            std::time::Duration::from_secs_f64(pts as f64 / frame.sample_rate as f64)
        } else {
            std::time::Duration::ZERO
        };

        // Calculate duration
        let sample_count = samples.len() / frame.channels;
        let duration =
            std::time::Duration::from_secs_f64(sample_count as f64 / frame.sample_rate as f64);

        Ok(AudioBuffer {
            format: AudioFormat::F32LE,
            sample_rate: frame.sample_rate as u32,
            channels: frame.channels as u8,
            samples,
            timestamp,
            duration,
        })
    }

    fn flush(&mut self) -> Result<Vec<AudioBuffer>, MediaError> {
        // minimp3 doesn't buffer frames, so nothing to flush
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mp3_decoder_creation() {
        let decoder = MP3Decoder::new();
        assert!(decoder.is_ok());
    }
}
