//! Unit tests for HardwareError enum

use cortenbrowser_hardware_accel::HardwareError;

#[test]
fn test_hardware_error_not_available_display() {
    let error = HardwareError::NotAvailable;
    assert_eq!(error.to_string(), "Hardware acceleration not available");
}

#[test]
fn test_hardware_error_unsupported_codec_display() {
    let error = HardwareError::UnsupportedCodec;
    assert_eq!(error.to_string(), "Codec not supported by hardware");
}

#[test]
fn test_hardware_error_initialization_failed_display() {
    let error = HardwareError::InitializationFailed;
    assert_eq!(error.to_string(), "Hardware decoder initialization failed");
}

#[test]
fn test_hardware_error_decode_failed_display() {
    let error = HardwareError::DecodeFailed;
    assert_eq!(error.to_string(), "Hardware decode operation failed");
}

#[test]
fn test_hardware_error_debug_format() {
    let error = HardwareError::NotAvailable;
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("NotAvailable"));
}

#[test]
fn test_hardware_error_equality() {
    assert_eq!(HardwareError::NotAvailable, HardwareError::NotAvailable);
    assert_ne!(HardwareError::NotAvailable, HardwareError::UnsupportedCodec);
}

#[test]
fn test_hardware_error_clone() {
    let error = HardwareError::InitializationFailed;
    let cloned = error.clone();
    assert_eq!(error, cloned);
}
