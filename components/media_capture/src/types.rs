//! Media capture type definitions
//!
//! This module defines types for device capture including constraints,
//! device information, and error types.

use std::fmt;

/// Constraints for video capture
///
/// # Examples
///
/// ```
/// use cortenbrowser_media_capture::CaptureConstraints;
///
/// let constraints = CaptureConstraints {
///     width: Some(1920),
///     height: Some(1080),
///     frame_rate: Some(30.0),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CaptureConstraints {
    /// Desired width in pixels
    pub width: Option<u32>,
    /// Desired height in pixels
    pub height: Option<u32>,
    /// Desired frame rate in frames per second
    pub frame_rate: Option<f32>,
}

/// Constraints for audio capture
///
/// # Examples
///
/// ```
/// use cortenbrowser_media_capture::AudioConstraints;
///
/// let constraints = AudioConstraints {
///     sample_rate: Some(48000),
///     channels: Some(2),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AudioConstraints {
    /// Desired sample rate in Hz
    pub sample_rate: Option<u32>,
    /// Desired number of audio channels
    pub channels: Option<u8>,
}

/// Kind of capture device
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    /// Video input device (camera)
    VideoInput,
    /// Audio input device (microphone)
    AudioInput,
    /// Audio output device (speakers)
    AudioOutput,
}

/// Information about a capture device
///
/// # Examples
///
/// ```
/// use cortenbrowser_media_capture::{DeviceInfo, DeviceKind};
///
/// let device = DeviceInfo {
///     device_id: "camera-001".to_string(),
///     label: "Built-in Camera".to_string(),
///     kind: DeviceKind::VideoInput,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceInfo {
    /// Unique device identifier
    pub device_id: String,
    /// Human-readable device label
    pub label: String,
    /// Type of device
    pub kind: DeviceKind,
}

/// Errors that can occur during media capture
#[derive(Debug, Clone, PartialEq)]
pub enum CaptureError {
    /// Requested device was not found
    DeviceNotFound,
    /// Permission to access device was denied
    PermissionDenied,
    /// Capture operation failed
    CaptureFailure,
}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaptureError::DeviceNotFound => write!(f, "Device not found"),
            CaptureError::PermissionDenied => write!(f, "Permission denied"),
            CaptureError::CaptureFailure => write!(f, "Capture failure"),
        }
    }
}

impl std::error::Error for CaptureError {}
