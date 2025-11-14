//! Unit tests for MicrophoneCapture
//!
//! Tests microphone capture functionality

use cortenbrowser_media_capture::{MicrophoneCapture, AudioConstraints};

#[test]
fn test_microphone_capture_new() {
    let device_id = "mic-001".to_string();
    let constraints = AudioConstraints {
        sample_rate: Some(48000),
        channels: Some(2),
    };

    let result = MicrophoneCapture::new(device_id, constraints);
    assert!(result.is_ok());
}

#[test]
fn test_microphone_capture_new_with_mono() {
    let device_id = "mic-002".to_string();
    let constraints = AudioConstraints {
        sample_rate: Some(44100),
        channels: Some(1),
    };

    let result = MicrophoneCapture::new(device_id, constraints);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_microphone_capture_start() {
    let device_id = "mic-001".to_string();
    let constraints = AudioConstraints {
        sample_rate: Some(48000),
        channels: Some(2),
    };

    let capture = MicrophoneCapture::new(device_id, constraints).unwrap();
    let result = capture.start().await;

    // Start should succeed (returns channel)
    assert!(result.is_ok());
}

#[test]
fn test_microphone_capture_stop() {
    let device_id = "mic-001".to_string();
    let constraints = AudioConstraints {
        sample_rate: Some(48000),
        channels: Some(2),
    };

    let capture = MicrophoneCapture::new(device_id, constraints).unwrap();
    let result = capture.stop();

    // Stop should succeed
    assert!(result.is_ok());
}
