//! Unit tests for error types

use cortenbrowser_shared_types::MediaError;

#[test]
fn test_unsupported_format_error() {
    let error = MediaError::UnsupportedFormat {
        format: "FLV".to_string(),
    };

    match error {
        MediaError::UnsupportedFormat { format } => {
            assert_eq!(format, "FLV");
        }
        _ => panic!("Expected UnsupportedFormat error"),
    }
}

#[test]
fn test_codec_error() {
    let error = MediaError::CodecError {
        details: "Failed to decode frame".to_string(),
    };

    match error {
        MediaError::CodecError { details } => {
            assert_eq!(details, "Failed to decode frame");
        }
        _ => panic!("Expected CodecError"),
    }
}

#[test]
fn test_network_error() {
    let error = MediaError::NetworkError {
        details: "Connection timeout".to_string(),
    };

    let error_str = format!("{}", error);
    assert!(error_str.contains("Network error"));
    assert!(error_str.contains("Connection timeout"));
}

#[test]
fn test_out_of_memory_error() {
    let error = MediaError::OutOfMemory;
    let error_str = format!("{}", error);
    assert!(error_str.contains("Out of memory"));
}

#[test]
fn test_error_debug() {
    let error = MediaError::DrmError {
        details: "License expired".to_string(),
    };
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("DrmError"));
}

#[test]
fn test_error_display() {
    let error = MediaError::HardwareError {
        details: "GPU unavailable".to_string(),
    };
    let display_str = format!("{}", error);
    assert!(display_str.contains("Hardware"));
}
