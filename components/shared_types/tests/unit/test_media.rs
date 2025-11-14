//! Unit tests for media data types

use cortenbrowser_shared_types::{
    AudioBuffer, AudioFormat, FrameMetadata, MediaSource, PixelFormat, SessionId, VideoFrame,
};
use std::time::Duration;

#[test]
fn test_session_id_creation() {
    let id1 = SessionId::new();
    let id2 = SessionId::new();

    assert_ne!(id1, id2);
}

#[test]
fn test_session_id_clone() {
    let id1 = SessionId::new();
    let id2 = id1.clone();

    assert_eq!(id1, id2);
}

#[test]
fn test_session_id_debug() {
    let id = SessionId::new();
    let debug_str = format!("{:?}", id);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_video_frame_creation() {
    let frame = VideoFrame {
        width: 1920,
        height: 1080,
        format: PixelFormat::YUV420,
        data: vec![0u8; 1920 * 1080],
        timestamp: Duration::from_secs(1),
        duration: Some(Duration::from_millis(33)),
        metadata: FrameMetadata::default(),
    };

    assert_eq!(frame.width, 1920);
    assert_eq!(frame.height, 1080);
    assert_eq!(frame.format, PixelFormat::YUV420);
    assert_eq!(frame.timestamp, Duration::from_secs(1));
    assert_eq!(frame.duration, Some(Duration::from_millis(33)));
}

#[test]
fn test_video_frame_clone() {
    let frame1 = VideoFrame {
        width: 640,
        height: 480,
        format: PixelFormat::RGB24,
        data: vec![0u8; 640 * 480 * 3],
        timestamp: Duration::from_millis(500),
        duration: None,
        metadata: FrameMetadata::default(),
    };

    let frame2 = frame1.clone();
    assert_eq!(frame1.width, frame2.width);
    assert_eq!(frame1.height, frame2.height);
    assert_eq!(frame1.format, frame2.format);
}

#[test]
fn test_audio_buffer_creation() {
    let buffer = AudioBuffer {
        format: AudioFormat::F32LE,
        sample_rate: 48000,
        channels: 2,
        samples: vec![0.0f32; 4800],
        timestamp: Duration::from_millis(100),
        duration: Duration::from_millis(100),
    };

    assert_eq!(buffer.format, AudioFormat::F32LE);
    assert_eq!(buffer.sample_rate, 48000);
    assert_eq!(buffer.channels, 2);
    assert_eq!(buffer.samples.len(), 4800);
}

#[test]
fn test_audio_buffer_clone() {
    let buffer1 = AudioBuffer {
        format: AudioFormat::S16LE,
        sample_rate: 44100,
        channels: 2,
        samples: vec![0.0f32; 1000],
        timestamp: Duration::from_secs(0),
        duration: Duration::from_millis(50),
    };

    let buffer2 = buffer1.clone();
    assert_eq!(buffer1.sample_rate, buffer2.sample_rate);
    assert_eq!(buffer1.channels, buffer2.channels);
}

#[test]
fn test_media_source_url() {
    let source = MediaSource::Url {
        url: "https://example.com/video.mp4".to_string(),
    };

    match source {
        MediaSource::Url { url } => {
            assert_eq!(url, "https://example.com/video.mp4");
        }
        _ => panic!("Expected Url variant"),
    }
}

#[test]
fn test_media_source_buffer() {
    let data = vec![1, 2, 3, 4, 5];
    let source = MediaSource::Buffer {
        data: data.clone(),
        mime_type: "video/mp4".to_string(),
    };

    match source {
        MediaSource::Buffer { data: d, mime_type } => {
            assert_eq!(d, data);
            assert_eq!(mime_type, "video/mp4");
        }
        _ => panic!("Expected Buffer variant"),
    }
}

#[test]
fn test_frame_metadata_default() {
    let metadata = FrameMetadata::default();
    let debug = format!("{:?}", metadata);
    assert!(!debug.is_empty());
}
