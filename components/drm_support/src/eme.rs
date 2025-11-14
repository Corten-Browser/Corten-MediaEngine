//! Encrypted Media Extensions (EME) Interface
//!
//! Provides the EME API for requesting media key system access and managing
//! DRM capabilities according to the W3C EME specification.

use crate::types::DrmError;
use serde::{Deserialize, Serialize};

/// Media Key System Configuration
///
/// Defines the configuration requirements for a media key system according
/// to EME specification.
///
/// # Examples
///
/// ```
/// use cortenbrowser_drm_support::MediaKeySystemConfiguration;
///
/// let config = MediaKeySystemConfiguration::default();
/// assert_eq!(config, MediaKeySystemConfiguration::default());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaKeySystemConfiguration {
    /// Initialization data types supported (e.g., "cenc", "keyids", "webm")
    pub init_data_types: Vec<String>,

    /// Audio capabilities
    pub audio_capabilities: Vec<MediaKeySystemMediaCapability>,

    /// Video capabilities
    pub video_capabilities: Vec<MediaKeySystemMediaCapability>,

    /// Distinctive identifier requirement
    pub distinctive_identifier: MediaKeysRequirement,

    /// Persistent state requirement
    pub persistent_state: MediaKeysRequirement,

    /// Session types supported
    pub session_types: Vec<String>,
}

impl Default for MediaKeySystemConfiguration {
    fn default() -> Self {
        Self {
            init_data_types: vec!["cenc".to_string(), "keyids".to_string()],
            audio_capabilities: vec![],
            video_capabilities: vec![],
            distinctive_identifier: MediaKeysRequirement::Optional,
            persistent_state: MediaKeysRequirement::Optional,
            session_types: vec!["temporary".to_string()],
        }
    }
}

/// Media Key System Media Capability
///
/// Describes a specific audio or video capability for DRM.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaKeySystemMediaCapability {
    /// Content type (MIME type with codecs)
    pub content_type: String,

    /// Robustness level required
    pub robustness: String,
}

/// Media Keys Requirement
///
/// Specifies the requirement level for a feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaKeysRequirement {
    /// Feature is required
    Required,

    /// Feature is optional
    Optional,

    /// Feature is not allowed
    NotAllowed,
}

/// Media Key System Access
///
/// Represents access to a specific key system configuration.
///
/// # Examples
///
/// ```
/// use cortenbrowser_drm_support::MediaKeySystemAccess;
///
/// let access = MediaKeySystemAccess::new("com.widevine.alpha".to_string());
/// assert_eq!(access.key_system(), "com.widevine.alpha");
/// ```
#[derive(Debug, Clone)]
pub struct MediaKeySystemAccess {
    /// Key system identifier
    key_system: String,

    /// Configuration that was accepted
    configuration: MediaKeySystemConfiguration,
}

impl MediaKeySystemAccess {
    /// Create a new MediaKeySystemAccess
    pub fn new(key_system: String) -> Self {
        Self {
            key_system,
            configuration: MediaKeySystemConfiguration::default(),
        }
    }

    /// Create with specific configuration
    pub fn with_configuration(key_system: String, configuration: MediaKeySystemConfiguration) -> Self {
        Self {
            key_system,
            configuration,
        }
    }

    /// Get the key system identifier
    pub fn key_system(&self) -> &str {
        &self.key_system
    }

    /// Get the configuration
    pub fn configuration(&self) -> &MediaKeySystemConfiguration {
        &self.configuration
    }
}

/// EME Interface
///
/// Provides the main entry point for EME operations, allowing applications
/// to request access to key systems.
///
/// # Examples
///
/// ```
/// use cortenbrowser_drm_support::{EMEInterface, MediaKeySystemConfiguration};
///
/// #[tokio::main]
/// async fn main() {
///     let eme = EMEInterface::new();
///     let configs = vec![MediaKeySystemConfiguration::default()];
///
///     let access = eme.request_media_key_system_access(
///         "com.widevine.alpha".to_string(),
///         configs
///     ).await.unwrap();
///
///     assert_eq!(access.key_system(), "com.widevine.alpha");
/// }
/// ```
pub struct EMEInterface {
    /// Supported key systems
    supported_key_systems: Vec<String>,
}

impl EMEInterface {
    /// Create a new EME interface
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_drm_support::EMEInterface;
    ///
    /// let eme = EMEInterface::new();
    /// ```
    pub fn new() -> Self {
        Self {
            // Stub implementation: In production, this would query
            // available CDMs on the platform
            supported_key_systems: vec![
                "com.widevine.alpha".to_string(),
                "com.microsoft.playready".to_string(),
                "com.apple.fps".to_string(),
                "org.w3.clearkey".to_string(),
                // Test key system for development
                "com.example.test".to_string(),
            ],
        }
    }

    /// Request media key system access
    ///
    /// Attempts to find a supported configuration for the requested key system.
    ///
    /// # Arguments
    ///
    /// * `key_system` - The key system identifier (e.g., "com.widevine.alpha")
    /// * `configs` - Array of supported configurations in priority order
    ///
    /// # Returns
    ///
    /// * `Ok(MediaKeySystemAccess)` - Access granted with selected configuration
    /// * `Err(DrmError::UnsupportedKeySystem)` - No supported configuration found
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_drm_support::{EMEInterface, MediaKeySystemConfiguration};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let eme = EMEInterface::new();
    ///
    ///     // Request access with multiple configurations
    ///     let configs = vec![
    ///         MediaKeySystemConfiguration::default(),
    ///     ];
    ///
    ///     let result = eme.request_media_key_system_access(
    ///         "com.widevine.alpha".to_string(),
    ///         configs
    ///     ).await;
    ///
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn request_media_key_system_access(
        &self,
        key_system: String,
        configs: Vec<MediaKeySystemConfiguration>,
    ) -> Result<MediaKeySystemAccess, DrmError> {
        // Stub implementation: In production, this would:
        // 1. Check if key system is supported on this platform
        // 2. Query CDM for supported configurations
        // 3. Match requested configs against CDM capabilities
        // 4. Select best matching configuration
        // 5. Return access object for creating MediaKeys

        // For stub, check if key system is in our supported list
        if !self.supported_key_systems.contains(&key_system) {
            return Err(DrmError::UnsupportedKeySystem(key_system));
        }

        // Use first configuration if available, otherwise use default
        let configuration = configs.into_iter().next()
            .unwrap_or_default();

        Ok(MediaKeySystemAccess::with_configuration(key_system, configuration))
    }

    /// Check if a key system is supported
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_drm_support::EMEInterface;
    ///
    /// let eme = EMEInterface::new();
    /// assert!(eme.is_key_system_supported("com.widevine.alpha"));
    /// assert!(!eme.is_key_system_supported("com.unknown.drm"));
    /// ```
    pub fn is_key_system_supported(&self, key_system: &str) -> bool {
        self.supported_key_systems.contains(&key_system.to_string())
    }

    /// Get list of supported key systems
    pub fn supported_key_systems(&self) -> &[String] {
        &self.supported_key_systems
    }
}

impl Default for EMEInterface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_key_system_configuration_default() {
        let config = MediaKeySystemConfiguration::default();
        assert_eq!(config.init_data_types.len(), 2);
        assert!(config.init_data_types.contains(&"cenc".to_string()));
        assert_eq!(config.distinctive_identifier, MediaKeysRequirement::Optional);
    }

    #[test]
    fn test_media_key_system_access() {
        let access = MediaKeySystemAccess::new("com.widevine.alpha".to_string());
        assert_eq!(access.key_system(), "com.widevine.alpha");
    }

    #[tokio::test]
    async fn test_eme_supported_key_systems() {
        let eme = EMEInterface::new();
        assert!(eme.is_key_system_supported("com.widevine.alpha"));
        assert!(eme.is_key_system_supported("org.w3.clearkey"));
        assert!(!eme.is_key_system_supported("com.unknown.system"));
    }

    #[tokio::test]
    async fn test_eme_request_access() {
        let eme = EMEInterface::new();
        let configs = vec![MediaKeySystemConfiguration::default()];

        let result = eme.request_media_key_system_access(
            "com.widevine.alpha".to_string(),
            configs
        ).await;

        assert!(result.is_ok());
        let access = result.unwrap();
        assert_eq!(access.key_system(), "com.widevine.alpha");
    }
}
