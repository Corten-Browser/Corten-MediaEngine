//! Core DRM types and error definitions
//!
//! This module defines the fundamental types used throughout the DRM support component.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for a DRM session
///
/// Each DRM session is identified by a unique session ID that is used to track
/// the session lifecycle from creation through license acquisition to decryption.
///
/// # Examples
///
/// ```
/// use cortenbrowser_drm_support::DrmSessionId;
///
/// // Create a new unique session ID
/// let session_id = DrmSessionId::new();
/// assert!(!session_id.as_str().is_empty());
///
/// // Create from an existing string
/// let session_id = DrmSessionId::from("existing-session-id".to_string());
/// assert_eq!(session_id.as_str(), "existing-session-id");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DrmSessionId(String);

impl DrmSessionId {
    /// Create a new unique DRM session ID
    ///
    /// Generates a UUID v4 and converts it to a string representation.
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Get the session ID as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for DrmSessionId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl Default for DrmSessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DrmSessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// DRM-specific errors
///
/// Represents all possible error conditions that can occur during DRM operations.
///
/// # Examples
///
/// ```
/// use cortenbrowser_drm_support::{DrmError, DrmSessionId};
///
/// let error = DrmError::UnsupportedKeySystem("com.widevine.alpha".to_string());
/// println!("Error: {}", error);
///
/// let session_error = DrmError::SessionNotFound(DrmSessionId::new());
/// assert!(matches!(session_error, DrmError::SessionNotFound(_)));
/// ```
#[derive(Debug, Clone, thiserror::Error)]
pub enum DrmError {
    /// The requested key system is not supported
    #[error("Unsupported key system: {0}")]
    UnsupportedKeySystem(String),

    /// License request to the server failed
    #[error("License request failed: {0}")]
    LicenseRequestFailed(String),

    /// Decryption of protected content failed
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// The requested DRM session was not found
    #[error("Session not found: {0}")]
    SessionNotFound(DrmSessionId),
}

/// Session types for DRM sessions
///
/// Defines the different types of DRM sessions according to EME specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SessionType {
    /// Temporary session - keys are not persisted
    #[default]
    Temporary,

    /// Persistent license session - keys can be stored for offline use
    PersistentLicense,

    /// Persistent release message session
    PersistentReleaseMessage,
}

/// DRM session state
///
/// Tracks the lifecycle state of a DRM session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SessionState {
    /// Session has been created but no license request generated
    #[default]
    Created,

    /// License request has been generated, waiting for response
    PendingLicense,

    /// License has been received and session is usable
    Active,

    /// Session has been closed
    Closed,

    /// Session encountered an error
    Error,
}

/// Internal session data
///
/// Stores the state and metadata for a DRM session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SessionData {
    /// Unique session identifier
    pub id: DrmSessionId,

    /// Current session state
    pub state: SessionState,

    /// Session type
    pub session_type: SessionType,

    /// Initialization data used for license request
    pub init_data: Option<Vec<u8>>,

    /// License data received from server
    pub license_data: Option<Vec<u8>>,
}

impl SessionData {
    /// Create new session data
    pub fn new(session_type: SessionType) -> Self {
        Self {
            id: DrmSessionId::new(),
            state: SessionState::Created,
            session_type,
            init_data: None,
            license_data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_data_creation() {
        let session = SessionData::new(SessionType::Temporary);
        assert_eq!(session.state, SessionState::Created);
        assert_eq!(session.session_type, SessionType::Temporary);
        assert!(session.init_data.is_none());
        assert!(session.license_data.is_none());
    }

    #[test]
    fn test_session_type_default() {
        assert_eq!(SessionType::default(), SessionType::Temporary);
    }

    #[test]
    fn test_session_state_default() {
        assert_eq!(SessionState::default(), SessionState::Created);
    }
}
