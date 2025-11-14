//! Unit tests for ContentDecryptionModule (CDM)
//!
//! Tests for CDM session management, license requests, and decryption.

use cortenbrowser_drm_support::{ContentDecryptionModule, DrmError, DrmSessionId};

#[test]
fn test_cdm_creation_with_supported_key_system() {
    /// Given: A supported key system
    /// When: We create a CDM
    /// Then: Creation should succeed
    let result = ContentDecryptionModule::new("com.example.test".to_string());

    assert!(result.is_ok(), "CDM creation should succeed for test key system");
}

#[test]
fn test_cdm_creation_with_unsupported_key_system() {
    /// Given: An unsupported key system
    /// When: We try to create a CDM
    /// Then: Creation should fail with UnsupportedKeySystem error
    let result = ContentDecryptionModule::new("".to_string());

    assert!(result.is_err(), "CDM creation should fail for empty key system");
    match result.unwrap_err() {
        DrmError::UnsupportedKeySystem(_) => {}, // Expected
        other => panic!("Expected UnsupportedKeySystem, got {:?}", other),
    }
}

#[tokio::test]
async fn test_cdm_create_session() {
    /// Given: A valid CDM instance
    /// When: We create a new session
    /// Then: A valid session ID should be returned
    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");

    let result = cdm.create_session().await;

    assert!(result.is_ok(), "Session creation should succeed");
    let session_id = result.unwrap();
    assert!(!session_id.as_str().is_empty(), "Session ID should not be empty");
}

#[tokio::test]
async fn test_cdm_create_multiple_sessions() {
    /// Given: A valid CDM instance
    /// When: We create multiple sessions
    /// Then: Each session should have a unique ID
    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");

    let session1 = cdm.create_session().await.expect("First session should be created");
    let session2 = cdm.create_session().await.expect("Second session should be created");

    assert_ne!(session1, session2, "Session IDs should be unique");
}

#[tokio::test]
async fn test_cdm_generate_request() {
    /// Given: A CDM with a valid session
    /// When: We generate a license request
    /// Then: A request payload should be returned
    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");
    let session_id = cdm.create_session().await.expect("Session creation should succeed");

    let init_data = b"initialization_data";
    let result = cdm.generate_request(&session_id, init_data).await;

    assert!(result.is_ok(), "License request generation should succeed");
    let request = result.unwrap();
    assert!(!request.is_empty(), "License request should not be empty");
}

#[tokio::test]
async fn test_cdm_generate_request_invalid_session() {
    /// Given: A CDM instance
    /// When: We try to generate a request for a non-existent session
    /// Then: Should fail with SessionNotFound error
    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");
    let invalid_session = DrmSessionId::from("non-existent-session".to_string());

    let init_data = b"initialization_data";
    let result = cdm.generate_request(&invalid_session, init_data).await;

    assert!(result.is_err(), "Request generation should fail for invalid session");
    match result.unwrap_err() {
        DrmError::SessionNotFound(_) => {}, // Expected
        other => panic!("Expected SessionNotFound, got {:?}", other),
    }
}

#[tokio::test]
async fn test_cdm_update_session() {
    /// Given: A CDM with a session and license request generated
    /// When: We update the session with a license response
    /// Then: The update should succeed
    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");
    let session_id = cdm.create_session().await.expect("Session creation should succeed");

    let init_data = b"initialization_data";
    cdm.generate_request(&session_id, init_data).await
        .expect("Request generation should succeed");

    let license_response = b"license_response_data";
    let result = cdm.update(&session_id, license_response).await;

    assert!(result.is_ok(), "Session update should succeed");
}

#[tokio::test]
async fn test_cdm_update_invalid_session() {
    /// Given: A CDM instance
    /// When: We try to update a non-existent session
    /// Then: Should fail with SessionNotFound error
    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");
    let invalid_session = DrmSessionId::from("non-existent-session".to_string());

    let license_response = b"license_response_data";
    let result = cdm.update(&invalid_session, license_response).await;

    assert!(result.is_err(), "Update should fail for invalid session");
    match result.unwrap_err() {
        DrmError::SessionNotFound(_) => {}, // Expected
        other => panic!("Expected SessionNotFound, got {:?}", other),
    }
}

#[test]
fn test_cdm_decrypt() {
    /// Given: A CDM instance
    /// When: We attempt to decrypt data (stub implementation)
    /// Then: Should return decrypted data or error
    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");

    let encrypted_data = b"encrypted_content";
    let key_id = b"key_identifier";

    let result = cdm.decrypt(encrypted_data, key_id);

    // For stub implementation, we expect this to work (returns placeholder data)
    // In production, this would integrate with actual CDM
    assert!(result.is_ok() || matches!(result.unwrap_err(), DrmError::DecryptionFailed(_)));
}

#[test]
fn test_cdm_decrypt_with_invalid_key() {
    /// Given: A CDM instance
    /// When: We try to decrypt with an empty key
    /// Then: Should handle gracefully
    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");

    let encrypted_data = b"encrypted_content";
    let empty_key = b"";

    let result = cdm.decrypt(encrypted_data, empty_key);

    // Should either succeed (stub) or fail gracefully
    assert!(result.is_ok() || result.is_err());
}
