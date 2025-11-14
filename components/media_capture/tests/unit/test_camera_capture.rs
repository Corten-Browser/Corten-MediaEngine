//! Unit tests for CameraCapture
//!
//! Tests camera capture functionality

use cortenbrowser_media_capture::{CameraCapture, CaptureConstraints};

#[test]
fn test_camera_capture_new() {
    let device_id = "camera-001".to_string();
    let constraints = CaptureConstraints {
        width: Some(1920),
        height: Some(1080),
        frame_rate: Some(30.0),
    };

    let result = CameraCapture::new(device_id, constraints);
    assert!(result.is_ok());
}

#[test]
fn test_camera_capture_new_with_empty_device_id() {
    let device_id = "".to_string();
    let constraints = CaptureConstraints {
        width: Some(640),
        height: Some(480),
        frame_rate: Some(15.0),
    };

    let result = CameraCapture::new(device_id, constraints);
    // Should still succeed (validation happens at start)
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_camera_capture_start() {
    let device_id = "camera-001".to_string();
    let constraints = CaptureConstraints {
        width: Some(1280),
        height: Some(720),
        frame_rate: Some(15.0),
    };

    let capture = CameraCapture::new(device_id, constraints).unwrap();
    let result = capture.start().await;

    // Start should succeed (returns channel)
    assert!(result.is_ok());
}

#[test]
fn test_camera_capture_stop() {
    let device_id = "camera-001".to_string();
    let constraints = CaptureConstraints {
        width: Some(1920),
        height: Some(1080),
        frame_rate: Some(30.0),
    };

    let capture = CameraCapture::new(device_id, constraints).unwrap();
    let result = capture.stop();

    // Stop should succeed
    assert!(result.is_ok());
}
