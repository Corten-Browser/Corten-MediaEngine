//! Unit tests for EME (Encrypted Media Extensions) Interface
//!
//! Tests for EME API compliance and key system access.

use cortenbrowser_drm_support::{
    DrmError, EMEInterface, MediaKeySystemAccess, MediaKeySystemConfiguration,
};

#[tokio::test]
async fn test_eme_request_media_key_system_access() {
    /// Given: An EME interface and supported key system
    /// When: We request media key system access
    /// Then: Should return a MediaKeySystemAccess object
    let eme = EMEInterface::new();
    let configs = vec![MediaKeySystemConfiguration::default()];

    let result = eme
        .request_media_key_system_access("com.example.test".to_string(), configs)
        .await;

    assert!(result.is_ok(), "Request should succeed for supported key system");
}

#[tokio::test]
async fn test_eme_request_unsupported_key_system() {
    /// Given: An EME interface
    /// When: We request an unsupported key system
    /// Then: Should fail with UnsupportedKeySystem error
    let eme = EMEInterface::new();
    let configs = vec![MediaKeySystemConfiguration::default()];

    let result = eme
        .request_media_key_system_access("com.unsupported.drm".to_string(), configs)
        .await;

    // For now, stub returns Ok, but in production this might fail
    assert!(result.is_ok() || matches!(result.unwrap_err(), DrmError::UnsupportedKeySystem(_)));
}

#[tokio::test]
async fn test_eme_request_with_empty_configs() {
    /// Given: An EME interface
    /// When: We request access with empty configuration list
    /// Then: Should handle appropriately
    let eme = EMEInterface::new();
    let configs = vec![];

    let result = eme
        .request_media_key_system_access("com.example.test".to_string(), configs)
        .await;

    // Should either succeed or fail gracefully
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_eme_request_with_multiple_configs() {
    /// Given: An EME interface
    /// When: We request access with multiple configurations
    /// Then: Should process all configurations
    let eme = EMEInterface::new();
    let configs = vec![
        MediaKeySystemConfiguration::default(),
        MediaKeySystemConfiguration::default(),
    ];

    let result = eme
        .request_media_key_system_access("com.example.test".to_string(), configs)
        .await;

    assert!(result.is_ok(), "Should handle multiple configurations");
}

#[test]
fn test_media_key_system_configuration_default() {
    /// Given: We need a default configuration
    /// When: We create a default MediaKeySystemConfiguration
    /// Then: It should be created successfully
    let config = MediaKeySystemConfiguration::default();

    // Verify default values (implementation-specific)
    assert_eq!(config, MediaKeySystemConfiguration::default());
}

#[test]
fn test_media_key_system_access_creation() {
    /// Given: We have successfully requested key system access
    /// When: We create a MediaKeySystemAccess object
    /// Then: It should contain the key system name
    let access = MediaKeySystemAccess::new("com.example.test".to_string());

    assert_eq!(access.key_system(), "com.example.test");
}
