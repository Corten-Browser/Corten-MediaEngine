//! Integration tests for complete pipeline flow
//!
//! Tests the end-to-end pipeline workflow.

use cortenbrowser_media_pipeline::{AVSyncController, MediaPipeline, PipelineConfig, SyncDecision};
use cortenbrowser_shared_types::{
    AudioBuffer, AudioFormat, FrameMetadata, MediaSource, PixelFormat, VideoFrame,
};
use std::time::Duration;

#[tokio::test]
async fn test_complete_pipeline_lifecycle() {
    // Given a MediaPipeline
    // When we go through a complete lifecycle
    // Then all state transitions should work correctly

    // Create pipeline with configuration
    let config = PipelineConfig {
        buffer_size: 2048,
        thread_count: 4,
        sync_threshold: Duration::from_millis(40),
    };

    let pipeline = MediaPipeline::new(config).unwrap();

    // Load a source
    let source = MediaSource::Url {
        url: "file:///test/video.mp4".to_string(),
    };
    pipeline.load_source(source).await.unwrap();

    // Start the pipeline
    pipeline.start().await.unwrap();

    // Seek to a position
    pipeline.seek(Duration::from_secs(5)).await.unwrap();

    // Stop the pipeline
    pipeline.stop().await.unwrap();
}

#[tokio::test]
async fn test_pipeline_with_buffer_source() {
    // Given a MediaPipeline
    // When loaded with a buffer source
    // Then it should handle the buffer data

    let pipeline = MediaPipeline::new(PipelineConfig::default()).unwrap();

    let source = MediaSource::Buffer {
        data: create_mock_mp4_data(),
        mime_type: "video/mp4".to_string(),
    };

    let result = pipeline.load_source(source).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_avsync_with_realistic_data() {
    // Given an AVSyncController and realistic A/V data
    // When processing frames with various timing scenarios
    // Then sync decisions should be appropriate

    let controller = AVSyncController::new();

    // Scenario 1: Perfect sync
    let frame1 = create_test_video_frame(Duration::from_millis(1000));
    let audio_ts1 = Duration::from_millis(1000);
    assert_eq!(
        controller.sync_frame(&frame1, audio_ts1),
        SyncDecision::Display
    );

    // Scenario 2: Video slightly behind (within tolerance)
    let frame2 = create_test_video_frame(Duration::from_millis(1980));
    let audio_ts2 = Duration::from_millis(2000);
    assert_eq!(
        controller.sync_frame(&frame2, audio_ts2),
        SyncDecision::Display
    );

    // Scenario 3: Video significantly behind (drop frame)
    let frame3 = create_test_video_frame(Duration::from_millis(2900));
    let audio_ts3 = Duration::from_millis(3000);
    assert_eq!(
        controller.sync_frame(&frame3, audio_ts3),
        SyncDecision::Drop
    );

    // Scenario 4: Video ahead (wait)
    let frame4 = create_test_video_frame(Duration::from_millis(4100));
    let audio_ts4 = Duration::from_millis(4000);
    match controller.sync_frame(&frame4, audio_ts4) {
        SyncDecision::Wait { duration } => {
            assert!(duration > Duration::ZERO);
            assert!(duration <= Duration::from_millis(100));
        }
        _ => panic!("Expected Wait decision"),
    }
}

#[tokio::test]
async fn test_pipeline_error_handling() {
    // Given a MediaPipeline
    // When invalid operations are attempted
    // Then appropriate errors should be returned

    let pipeline = MediaPipeline::new(PipelineConfig::default()).unwrap();

    // Try to start without loading source
    let result = pipeline.start().await;
    assert!(result.is_err());

    // Load source
    let source = MediaSource::Url {
        url: "file:///test/video.mp4".to_string(),
    };
    pipeline.load_source(source).await.unwrap();

    // Start successfully
    pipeline.start().await.unwrap();

    // Try to load another source while running (should fail)
    let source2 = MediaSource::Url {
        url: "file:///test/video2.mp4".to_string(),
    };
    let result = pipeline.load_source(source2).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_frame_queue_operations() {
    // Given a MediaPipeline
    // When checking for frames before any are available
    // Then it should return None appropriately

    let pipeline = MediaPipeline::new(PipelineConfig::default()).unwrap();

    // Initially, no frames should be available
    assert!(pipeline.get_next_video_frame().await.is_none());
    assert!(pipeline.get_next_audio_buffer().await.is_none());
}

// Helper functions

fn create_test_video_frame(timestamp: Duration) -> VideoFrame {
    VideoFrame {
        width: 1920,
        height: 1080,
        format: PixelFormat::YUV420,
        data: vec![0u8; 1920 * 1080],
        timestamp,
        duration: Some(Duration::from_millis(33)),
        metadata: FrameMetadata {
            is_keyframe: true,
            pts: Some(timestamp.as_millis() as i64),
            dts: Some(timestamp.as_millis() as i64),
            sequence: Some(0),
        },
    }
}

#[allow(dead_code)]
fn create_test_audio_buffer(timestamp: Duration) -> AudioBuffer {
    AudioBuffer {
        format: AudioFormat::F32LE,
        sample_rate: 48000,
        channels: 2,
        samples: vec![0.0f32; 4800],
        timestamp,
        duration: Duration::from_millis(100),
    }
}

fn create_mock_mp4_data() -> Vec<u8> {
    // Minimal MP4 header for testing
    // In a real implementation, this would be actual MP4 data
    vec![0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70] // ftyp box
}
