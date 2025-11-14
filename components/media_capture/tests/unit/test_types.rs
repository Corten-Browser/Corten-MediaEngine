//! Unit tests for media capture types
//!
//! Tests for CaptureConstraints, AudioConstraints, DeviceInfo, DeviceKind, and CaptureError

use cortenbrowser_media_capture::*;

#[test]
fn test_capture_constraints_creation() {
    let constraints = CaptureConstraints {
        width: Some(1920),
        height: Some(1080),
        frame_rate: Some(30.0),
    };

    assert_eq!(constraints.width, Some(1920));
    assert_eq!(constraints.height, Some(1080));
    assert_eq!(constraints.frame_rate, Some(30.0));
}

#[test]
fn test_capture_constraints_optional_fields() {
    let constraints = CaptureConstraints {
        width: None,
        height: None,
        frame_rate: None,
    };

    assert_eq!(constraints.width, None);
    assert_eq!(constraints.height, None);
    assert_eq!(constraints.frame_rate, None);
}

#[test]
fn test_audio_constraints_creation() {
    let constraints = AudioConstraints {
        sample_rate: Some(48000),
        channels: Some(2),
    };

    assert_eq!(constraints.sample_rate, Some(48000));
    assert_eq!(constraints.channels, Some(2));
}

#[test]
fn test_audio_constraints_optional_fields() {
    let constraints = AudioConstraints {
        sample_rate: None,
        channels: None,
    };

    assert_eq!(constraints.sample_rate, None);
    assert_eq!(constraints.channels, None);
}

#[test]
fn test_device_kind_variants() {
    let video_input = DeviceKind::VideoInput;
    let audio_input = DeviceKind::AudioInput;
    let audio_output = DeviceKind::AudioOutput;

    // Test that variants can be compared
    assert_ne!(video_input, audio_input);
    assert_ne!(audio_input, audio_output);
    assert_ne!(video_input, audio_output);
}

#[test]
fn test_device_info_creation() {
    let device = DeviceInfo {
        device_id: "camera-001".to_string(),
        label: "Built-in Camera".to_string(),
        kind: DeviceKind::VideoInput,
    };

    assert_eq!(device.device_id, "camera-001");
    assert_eq!(device.label, "Built-in Camera");
    assert_eq!(device.kind, DeviceKind::VideoInput);
}

#[test]
fn test_capture_error_variants() {
    let not_found = CaptureError::DeviceNotFound;
    let permission_denied = CaptureError::PermissionDenied;
    let capture_failure = CaptureError::CaptureFailure;

    // Test that error variants can be created
    assert!(matches!(not_found, CaptureError::DeviceNotFound));
    assert!(matches!(permission_denied, CaptureError::PermissionDenied));
    assert!(matches!(capture_failure, CaptureError::CaptureFailure));
}

#[test]
fn test_capture_error_display() {
    let error = CaptureError::DeviceNotFound;
    let error_str = format!("{}", error);
    assert!(!error_str.is_empty());
}
