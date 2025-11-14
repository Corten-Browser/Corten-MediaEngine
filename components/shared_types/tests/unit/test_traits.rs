//! Unit tests for trait definitions

use cortenbrowser_shared_types::{
    AudioBuffer, AudioDecoder, AudioPacket, Demuxer, MediaError, MediaInfo, MediaSource, SessionId,
    VideoDecoder, VideoFrame, VideoPacket,
};
use std::time::Duration;
use tokio::sync::mpsc;

// Mock implementation for testing trait bounds
struct MockDemuxer;

impl Demuxer for MockDemuxer {
    fn parse(&self, _data: &[u8]) -> Result<MediaInfo, MediaError> {
        Ok(MediaInfo::default())
    }

    fn get_video_packets(&self) -> mpsc::Receiver<VideoPacket> {
        let (_tx, rx) = mpsc::channel(10);
        rx
    }

    fn get_audio_packets(&self) -> mpsc::Receiver<AudioPacket> {
        let (_tx, rx) = mpsc::channel(10);
        rx
    }
}

struct MockVideoDecoder;

impl VideoDecoder for MockVideoDecoder {
    fn decode(&mut self, _packet: &VideoPacket) -> Result<VideoFrame, MediaError> {
        Err(MediaError::CodecError {
            details: "Not implemented".to_string(),
        })
    }

    fn flush(&mut self) -> Result<Vec<VideoFrame>, MediaError> {
        Ok(Vec::new())
    }
}

struct MockAudioDecoder;

impl AudioDecoder for MockAudioDecoder {
    fn decode(&mut self, _packet: &AudioPacket) -> Result<AudioBuffer, MediaError> {
        Err(MediaError::CodecError {
            details: "Not implemented".to_string(),
        })
    }

    fn flush(&mut self) -> Result<Vec<AudioBuffer>, MediaError> {
        Ok(Vec::new())
    }
}

#[test]
fn test_demuxer_trait_impl() {
    let demuxer = MockDemuxer;
    let data = vec![0u8; 100];
    let result = demuxer.parse(&data);
    assert!(result.is_ok());
}

#[test]
fn test_demuxer_get_packets() {
    let demuxer = MockDemuxer;
    let _video_rx = demuxer.get_video_packets();
    let _audio_rx = demuxer.get_audio_packets();
}

#[test]
fn test_video_decoder_trait_impl() {
    let mut decoder = MockVideoDecoder;
    let packet = VideoPacket::default();
    let result = decoder.decode(&packet);
    assert!(result.is_err());
}

#[test]
fn test_video_decoder_flush() {
    let mut decoder = MockVideoDecoder;
    let result = decoder.flush();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_audio_decoder_trait_impl() {
    let mut decoder = MockAudioDecoder;
    let packet = AudioPacket::default();
    let result = decoder.decode(&packet);
    assert!(result.is_err());
}

#[test]
fn test_audio_decoder_flush() {
    let mut decoder = MockAudioDecoder;
    let result = decoder.flush();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_trait_object_demuxer() {
    let demuxer: Box<dyn Demuxer> = Box::new(MockDemuxer);
    let data = vec![0u8; 100];
    let result = demuxer.parse(&data);
    assert!(result.is_ok());
}

#[test]
fn test_trait_object_video_decoder() {
    let mut decoder: Box<dyn VideoDecoder> = Box::new(MockVideoDecoder);
    let frames = decoder.flush().unwrap();
    assert_eq!(frames.len(), 0);
}
