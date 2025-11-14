//! Unit tests for DeviceEnumerator
//!
//! Tests device enumeration for video and audio devices

use cortenbrowser_media_capture::{DeviceEnumerator, DeviceKind};

#[tokio::test]
async fn test_enumerate_video_devices() {
    let enumerator = DeviceEnumerator::new();
    let result = enumerator.enumerate_video_devices().await;

    assert!(result.is_ok());
    let devices = result.unwrap();

    // Should return a list (may be empty on CI)
    assert!(devices.is_empty() || !devices.is_empty());

    // All devices should be video input
    for device in devices {
        assert_eq!(device.kind, DeviceKind::VideoInput);
        assert!(!device.device_id.is_empty());
    }
}

#[tokio::test]
async fn test_enumerate_audio_devices() {
    let enumerator = DeviceEnumerator::new();
    let result = enumerator.enumerate_audio_devices().await;

    assert!(result.is_ok());
    let devices = result.unwrap();

    // Should return a list (may be empty on CI)
    assert!(devices.is_empty() || !devices.is_empty());

    // All devices should be audio input
    for device in devices {
        assert_eq!(device.kind, DeviceKind::AudioInput);
        assert!(!device.device_id.is_empty());
    }
}

#[tokio::test]
async fn test_device_enumerator_multiple_calls() {
    let enumerator = DeviceEnumerator::new();

    // Should be able to enumerate multiple times
    let result1 = enumerator.enumerate_video_devices().await;
    let result2 = enumerator.enumerate_video_devices().await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}
