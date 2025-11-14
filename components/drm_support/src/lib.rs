//! # drm_support Component
//!
//! DRM and Encrypted Media Extensions (CDM interface, license acquisition, secure decryption)
//!
//! This component provides DRM (Digital Rights Management) support for the Corten Media Engine,
//! including:
//!
//! - Content Decryption Module (CDM) interface for managing DRM sessions
//! - EME (Encrypted Media Extensions) API for requesting key system access
//! - License acquisition and session management
//! - Decryption interface (stub implementation - production requires platform CDM)
//!
//! # Architecture
//!
//! The DRM support component follows the W3C Encrypted Media Extensions specification:
//!
//! 1. **EME Interface**: Entry point for requesting access to key systems
//! 2. **Content Decryption Module (CDM)**: Manages DRM sessions and decryption
//! 3. **Session Management**: Tracks DRM session lifecycle from creation to decryption
//!
//! # Examples
//!
//! ## Complete DRM Workflow
//!
//! ```rust
//! use cortenbrowser_drm_support::{
//!     ContentDecryptionModule, EMEInterface, MediaKeySystemConfiguration
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Step 1: Request key system access via EME
//!     let eme = EMEInterface::new();
//!     let configs = vec![MediaKeySystemConfiguration::default()];
//!     let access = eme.request_media_key_system_access(
//!         "com.widevine.alpha".to_string(),
//!         configs
//!     ).await?;
//!
//!     // Step 2: Create CDM for the key system
//!     let cdm = ContentDecryptionModule::new(access.key_system().to_string())?;
//!
//!     // Step 3: Create a DRM session
//!     let session_id = cdm.create_session().await?;
//!
//!     // Step 4: Generate license request
//!     let init_data = b"initialization_data_from_media";
//!     let license_request = cdm.generate_request(&session_id, init_data).await?;
//!
//!     // Step 5: (Send license_request to license server and get response)
//!     let license_response = b"license_from_server";
//!
//!     // Step 6: Update session with license
//!     cdm.update(&session_id, license_response).await?;
//!
//!     // Step 7: Decrypt content
//!     let encrypted_data = b"encrypted_content";
//!     let key_id = b"key_identifier";
//!     let decrypted_data = cdm.decrypt(encrypted_data, key_id)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Security Considerations
//!
//! **IMPORTANT**: This is a stub implementation for development and testing.
//!
//! Production implementation MUST:
//! - Use platform-specific CDM (Widevine, PlayReady, FairPlay)
//! - Implement secure decryption in hardware TEE/secure enclave
//! - Never expose clear decryption keys in memory
//! - Validate HDCP and output protection requirements
//! - Implement secure video path for protected content
//!
//! See `docs/DRM-SECURITY-REQUIREMENTS.md` for detailed security requirements.

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Module declarations
mod cdm;
mod eme;
mod types;

// Re-export public API
pub use cdm::ContentDecryptionModule;
pub use eme::{
    EMEInterface, MediaKeySystemAccess, MediaKeySystemConfiguration,
    MediaKeySystemMediaCapability, MediaKeysRequirement,
};
pub use types::{DrmError, DrmSessionId, SessionState, SessionType};
