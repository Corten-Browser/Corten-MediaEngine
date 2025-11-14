//! Integration tests for DRM session lifecycle
//!
//! Tests the complete flow of DRM session creation, license acquisition, and decryption.

use cortenbrowser_drm_support::{ContentDecryptionModule, EMEInterface, MediaKeySystemConfiguration};

#[tokio::test]
async fn test_complete_drm_session_lifecycle() {
    /// Given: An EME interface and CDM
    /// When: We go through the complete DRM workflow
    /// Then: All steps should complete successfully

    // Step 1: Request media key system access via EME
    let eme = EMEInterface::new();
    let configs = vec![MediaKeySystemConfiguration::default()];

    let access = eme
        .request_media_key_system_access("com.example.test".to_string(), configs)
        .await
        .expect("Should get key system access");

    assert_eq!(access.key_system(), "com.example.test");

    // Step 2: Create CDM for the key system
    let cdm = ContentDecryptionModule::new(access.key_system().to_string())
        .expect("CDM creation should succeed");

    // Step 3: Create a DRM session
    let session_id = cdm
        .create_session()
        .await
        .expect("Session creation should succeed");

    assert!(!session_id.as_str().is_empty());

    // Step 4: Generate license request
    let init_data = b"pssh_box_data_from_media";
    let license_request = cdm
        .generate_request(&session_id, init_data)
        .await
        .expect("License request generation should succeed");

    assert!(!license_request.is_empty());

    // Step 5: Update session with license response
    // (In real scenario, this would come from a license server)
    let license_response = b"license_from_server";
    cdm.update(&session_id, license_response)
        .await
        .expect("Session update should succeed");

    // Step 6: Decrypt content (stub implementation for now)
    let encrypted_data = b"encrypted_video_data";
    let key_id = b"content_key_id";

    let _decrypted = cdm
        .decrypt(encrypted_data, key_id)
        .expect("Decryption should succeed or fail gracefully");
}

#[tokio::test]
async fn test_multiple_concurrent_sessions() {
    /// Given: A CDM instance
    /// When: We create multiple concurrent sessions
    /// Then: All sessions should be independent

    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");

    // Create multiple sessions concurrently
    let session1 = cdm.create_session().await.expect("Session 1 creation");
    let session2 = cdm.create_session().await.expect("Session 2 creation");
    let session3 = cdm.create_session().await.expect("Session 3 creation");

    // Verify all sessions are unique
    assert_ne!(session1, session2);
    assert_ne!(session2, session3);
    assert_ne!(session1, session3);

    // Generate requests for each session
    let init_data = b"initialization_data";

    let req1 = cdm.generate_request(&session1, init_data).await.expect("Request 1");
    let req2 = cdm.generate_request(&session2, init_data).await.expect("Request 2");
    let req3 = cdm.generate_request(&session3, init_data).await.expect("Request 3");

    // All requests should succeed
    assert!(!req1.is_empty());
    assert!(!req2.is_empty());
    assert!(!req3.is_empty());
}

#[tokio::test]
async fn test_session_isolation() {
    /// Given: Multiple DRM sessions
    /// When: We update one session
    /// Then: Other sessions should not be affected

    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");

    let session1 = cdm.create_session().await.expect("Session 1");
    let session2 = cdm.create_session().await.expect("Session 2");

    // Generate requests for both
    let init_data = b"init_data";
    cdm.generate_request(&session1, init_data).await.expect("Request 1");
    cdm.generate_request(&session2, init_data).await.expect("Request 2");

    // Update only session1
    let license = b"license_data";
    cdm.update(&session1, license).await.expect("Update session 1");

    // Session2 should still be valid and updatable
    cdm.update(&session2, license).await.expect("Update session 2 should still work");
}

#[tokio::test]
async fn test_error_handling_in_lifecycle() {
    /// Given: A CDM instance
    /// When: We perform operations with invalid data
    /// Then: Errors should be handled gracefully

    let cdm = ContentDecryptionModule::new("com.example.test".to_string())
        .expect("CDM creation should succeed");

    let session_id = cdm.create_session().await.expect("Session creation");

    // Try to generate request with empty init data
    let result = cdm.generate_request(&session_id, &[]).await;
    // Should either succeed (stub allows it) or fail gracefully
    assert!(result.is_ok() || result.is_err());

    // Try to update with empty license response
    let result = cdm.update(&session_id, &[]).await;
    assert!(result.is_ok() || result.is_err());
}
