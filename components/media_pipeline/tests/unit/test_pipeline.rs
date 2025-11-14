//! Unit tests for MediaPipeline
//!
//! Tests the pipeline orchestration logic.

use cortenbrowser_media_pipeline::{MediaPipeline, PipelineConfig};
use cortenbrowser_shared_types::{MediaError, MediaSource};
use std::time::Duration;

#[tokio::test]
async fn test_pipeline_creation_with_config() {
    //
    Given a PipelineConfig
    When a new MediaPipeline is created
    Then it should initialize successfully
    //
    let config = PipelineConfig::default();
    let result = MediaPipeline::new(config);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pipeline_load_url_source() {
    //
    Given a MediaPipeline
    When load_source is called with a URL
    Then it should accept the source without error
    //
    let config = PipelineConfig::default();
    let pipeline = MediaPipeline::new(config).unwrap();

    let source = MediaSource::Url {
        url: "file:///test/video.mp4".to_string(),
    };

    let result = pipeline.load_source(source).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pipeline_load_buffer_source() {
    //
    Given a MediaPipeline
    When load_source is called with a buffer
    Then it should accept the source without error
    //
    let config = PipelineConfig::default();
    let pipeline = MediaPipeline::new(config).unwrap();

    let source = MediaSource::Buffer {
        data: vec![0u8; 1024],
        mime_type: "video/mp4".to_string(),
    };

    let result = pipeline.load_source(source).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pipeline_start_without_source_fails() {
    //
    Given a MediaPipeline with no source loaded
    When start is called
    Then it should return an error
    //
    let config = PipelineConfig::default();
    let pipeline = MediaPipeline::new(config).unwrap();

    let result = pipeline.start().await;
    assert!(result.is_err());

    if let Err(MediaError::InvalidStateTransition { .. }) = result {
        // Expected error type
    } else {
        panic!("Expected InvalidStateTransition error");
    }
}

#[tokio::test]
async fn test_pipeline_start_after_load_succeeds() {
    //
    Given a MediaPipeline with a source loaded
    When start is called
    Then it should transition to running state
    //
    let config = PipelineConfig::default();
    let pipeline = MediaPipeline::new(config).unwrap();

    let source = MediaSource::Url {
        url: "file:///test/video.mp4".to_string(),
    };

    pipeline.load_source(source).await.unwrap();
    let result = pipeline.start().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pipeline_stop() {
    //
    Given a running MediaPipeline
    When stop is called
    Then it should transition to stopped state
    //
    let config = PipelineConfig::default();
    let pipeline = MediaPipeline::new(config).unwrap();

    let source = MediaSource::Url {
        url: "file:///test/video.mp4".to_string(),
    };

    pipeline.load_source(source).await.unwrap();
    pipeline.start().await.unwrap();

    let result = pipeline.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pipeline_seek() {
    //
    Given a running MediaPipeline
    When seek is called with a position
    Then it should accept the seek position
    //
    let config = PipelineConfig::default();
    let pipeline = MediaPipeline::new(config).unwrap();

    let source = MediaSource::Url {
        url: "file:///test/video.mp4".to_string(),
    };

    pipeline.load_source(source).await.unwrap();
    pipeline.start().await.unwrap();

    let seek_position = Duration::from_secs(10);
    let result = pipeline.seek(seek_position).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pipeline_get_video_frame_returns_none_initially() {
    //
    Given a MediaPipeline
    When get_next_video_frame is called before data is available
    Then it should return None
    //
    let config = PipelineConfig::default();
    let pipeline = MediaPipeline::new(config).unwrap();

    let frame = pipeline.get_next_video_frame().await;
    assert!(frame.is_none());
}

#[tokio::test]
async fn test_pipeline_get_audio_buffer_returns_none_initially() {
    //
    Given a MediaPipeline
    When get_next_audio_buffer is called before data is available
    Then it should return None
    //
    let config = PipelineConfig::default();
    let pipeline = MediaPipeline::new(config).unwrap();

    let buffer = pipeline.get_next_audio_buffer().await;
    assert!(buffer.is_none());
}

#[tokio::test]
async fn test_pipeline_config_defaults() {
    //
    Given PipelineConfig default constructor
    When values are checked
    Then they should match expected defaults
    //
    let config = PipelineConfig::default();

    assert_eq!(config.buffer_size, 1024);
    assert_eq!(config.thread_count, 4);
    assert_eq!(config.sync_threshold, Duration::from_millis(40));
}

#[tokio::test]
async fn test_pipeline_config_custom_values() {
    //
    Given a custom PipelineConfig
    When the pipeline is created with it
    Then the configuration should be applied
    //
    let config = PipelineConfig {
        buffer_size: 2048,
        thread_count: 8,
        sync_threshold: Duration::from_millis(50),
    };

    let result = MediaPipeline::new(config);
    assert!(result.is_ok());
}
