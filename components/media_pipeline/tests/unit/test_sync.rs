//! Unit tests for AVSyncController
//!
//! Tests the audio/video synchronization logic.

use cortenbrowser_media_pipeline::{AVSyncController, SyncDecision};
use cortenbrowser_shared_types::{AudioBuffer, VideoFrame, PixelFormat, AudioFormat, FrameMetadata};
use std::time::Duration;

#[test]
fn test_avsync_controller_creation() {
    //
    Given a new AVSyncController is created
    When we create it with default settings
    Then it should initialize successfully with a clock at zero
    //
    let controller = AVSyncController::new();
    assert_eq!(controller.get_clock(), Duration::ZERO);
}

#[test]
fn test_sync_decision_display_when_in_sync() {
    //
    Given a video frame and audio at the same timestamp
    When sync_frame is called
    Then it should return Display decision
    //
    let controller = AVSyncController::new();

    let video_frame = create_test_video_frame(Duration::from_millis(1000));
    let audio_timestamp = Duration::from_millis(1000);

    let decision = controller.sync_frame(&video_frame, audio_timestamp);

    assert_eq!(decision, SyncDecision::Display);
}

#[test]
fn test_sync_decision_wait_when_video_ahead() {
    //
    Given a video frame ahead of audio
    When sync_frame is called
    Then it should return Wait decision with appropriate duration
    //
    let controller = AVSyncController::new();

    // Video is 50ms ahead of audio
    let video_frame = create_test_video_frame(Duration::from_millis(1050));
    let audio_timestamp = Duration::from_millis(1000);

    let decision = controller.sync_frame(&video_frame, audio_timestamp);

    match decision {
        SyncDecision::Wait { duration } => {
            assert!(duration > Duration::ZERO);
            assert!(duration <= Duration::from_millis(50));
        }
        _ => panic!("Expected Wait decision, got {:?}", decision),
    }
}

#[test]
fn test_sync_decision_drop_when_video_behind() {
    //
    Given a video frame significantly behind audio (>50ms)
    When sync_frame is called
    Then it should return Drop decision to catch up
    //
    let controller = AVSyncController::new();

    // Video is 100ms behind audio (beyond threshold)
    let video_frame = create_test_video_frame(Duration::from_millis(900));
    let audio_timestamp = Duration::from_millis(1000));

    let decision = controller.sync_frame(&video_frame, audio_timestamp);

    assert_eq!(decision, SyncDecision::Drop);
}

#[test]
fn test_sync_threshold_tolerance() {
    //
    Given a video frame slightly out of sync (within threshold)
    When sync_frame is called
    Then it should still display (within tolerance)
    //
    let controller = AVSyncController::new();

    // Video is 20ms behind (within typical 40ms tolerance)
    let video_frame = create_test_video_frame(Duration::from_millis(980));
    let audio_timestamp = Duration::from_millis(1000));

    let decision = controller.sync_frame(&video_frame, audio_timestamp);

    // Should display because it's within tolerance
    assert_eq!(decision, SyncDecision::Display);
}

#[test]
fn test_clock_advancement() {
    //
    Given an AVSyncController
    When frames are processed over time
    Then the clock should advance appropriately
    //
    let controller = AVSyncController::new();

    // Initially at zero
    assert_eq!(controller.get_clock(), Duration::ZERO);

    // Process a frame at 1 second
    let video_frame = create_test_video_frame(Duration::from_secs(1));
    let audio_timestamp = Duration::from_secs(1);
    controller.sync_frame(&video_frame, audio_timestamp);

    // Clock should have advanced
    let clock = controller.get_clock();
    assert!(clock >= Duration::from_secs(1));
}

// Helper function to create test video frames
fn create_test_video_frame(timestamp: Duration) -> VideoFrame {
    VideoFrame {
        width: 1920,
        height: 1080,
        format: PixelFormat::YUV420,
        data: vec![0u8; 1920 * 1080],
        timestamp,
        duration: Some(Duration::from_millis(33)), // ~30fps
        metadata: FrameMetadata::default(),
    }
}

// Helper function to create test audio buffers
#[allow(dead_code)]
fn create_test_audio_buffer(timestamp: Duration) -> AudioBuffer {
    AudioBuffer {
        format: AudioFormat::F32LE,
        sample_rate: 48000,
        channels: 2,
        samples: vec![0.0f32; 4800], // 100ms of audio
        timestamp,
        duration: Duration::from_millis(100),
    }
}
