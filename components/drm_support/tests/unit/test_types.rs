//! Unit tests for DRM types
//!
//! Tests for DrmSessionId, DrmError, and related type definitions.

use cortenbrowser_drm_support::{DrmError, DrmSessionId};

#[test]
fn test_drm_session_id_creation() {
    /// Given: We want to create a new DRM session ID
    /// When: We create a DrmSessionId from a String
    /// Then: The session ID should be created successfully
    let session_id = DrmSessionId::new();
    let id_str = session_id.as_str();

    assert!(!id_str.is_empty(), "Session ID should not be empty");
}

#[test]
fn test_drm_session_id_from_string() {
    /// Given: A valid session ID string
    /// When: We create a DrmSessionId from it
    /// Then: The session ID should contain the correct value
    let id_string = "test-session-123".to_string();
    let session_id = DrmSessionId::from(id_string.clone());

    assert_eq!(session_id.as_str(), &id_string);
}

#[test]
fn test_drm_session_id_equality() {
    /// Given: Two session IDs with the same value
    /// When: We compare them
    /// Then: They should be equal
    let id1 = DrmSessionId::from("session-1".to_string());
    let id2 = DrmSessionId::from("session-1".to_string());
    let id3 = DrmSessionId::from("session-2".to_string());

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_drm_error_unsupported_key_system() {
    /// Given: An unsupported key system error
    /// When: We create and format the error
    /// Then: The error message should be appropriate
    let error = DrmError::UnsupportedKeySystem("com.widevine.alpha".to_string());
    let error_msg = format!("{}", error);

    assert!(error_msg.contains("com.widevine.alpha"));
    assert!(error_msg.contains("unsupported") || error_msg.contains("Unsupported"));
}

#[test]
fn test_drm_error_license_request_failed() {
    // Given: A license request failure
    // When: We create the error
    // Then: The error should be of the correct variant
    let error = DrmError::LicenseRequestFailed("Network timeout".to_string());

    match error {
        DrmError::LicenseRequestFailed(_) => {}, // Expected
        _ => panic!("Expected LicenseRequestFailed error"),
    }
}

#[test]
fn test_drm_error_decryption_failed() {
    // Given: A decryption failure
    // When: We create the error
    // Then: The error should be of the correct variant
    let error = DrmError::DecryptionFailed("Invalid key".to_string());

    match error {
        DrmError::DecryptionFailed(_) => {}, // Expected
        _ => panic!("Expected DecryptionFailed error"),
    }
}

#[test]
fn test_drm_error_session_not_found() {
    // Given: A session not found error
    // When: We create the error with a session ID
    // Then: The error should contain the session ID
    let session_id = DrmSessionId::new();
    let error = DrmError::SessionNotFound(session_id.clone());

    match error {
        DrmError::SessionNotFound(id) => {
            assert_eq!(id, session_id);
        },
        _ => panic!("Expected SessionNotFound error"),
    }
}
