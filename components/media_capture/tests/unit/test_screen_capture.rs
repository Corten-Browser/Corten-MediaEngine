//! Unit tests for ScreenCapture
//!
//! Tests screen capture functionality

use cortenbrowser_media_capture::{ScreenCapture, CaptureConstraints};

#[test]
fn test_screen_capture_new() {
    let constraints = CaptureConstraints {
        width: Some(1920),
        height: Some(1080),
        frame_rate: Some(30.0),
    };

    let result = ScreenCapture::new(constraints);
    assert!(result.is_ok());
}

#[test]
fn test_screen_capture_new_with_none_constraints() {
    let constraints = CaptureConstraints {
        width: None,
        height: None,
        frame_rate: None,
    };

    let result = ScreenCapture::new(constraints);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_screen_capture_start() {
    let constraints = CaptureConstraints {
        width: Some(1280),
        height: Some(720),
        frame_rate: Some(15.0),
    };

    let capture = ScreenCapture::new(constraints).unwrap();
    let result = capture.start().await;

    // Start should succeed (returns channel)
    assert!(result.is_ok());
}

#[test]
fn test_screen_capture_stop() {
    let constraints = CaptureConstraints {
        width: Some(1920),
        height: Some(1080),
        frame_rate: Some(30.0),
    };

    let capture = ScreenCapture::new(constraints).unwrap();
    let result = capture.stop();

    // Stop should succeed
    assert!(result.is_ok());
}
