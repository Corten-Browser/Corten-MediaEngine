//! Session management types
//!
//! This module provides types for managing media playback sessions.

use std::fmt;
use uuid::Uuid;

/// Unique identifier for a media session
///
/// Each media playback session is assigned a unique ID for tracking
/// and managing its lifecycle.
///
/// # Examples
///
/// ```
/// use cortenbrowser_shared_types::SessionId;
///
/// let id1 = SessionId::new();
/// let id2 = SessionId::new();
/// assert_ne!(id1, id2); // Each ID is unique
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(Uuid);

impl SessionId {
    /// Creates a new random session ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a session ID from a UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the underlying UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SessionId({})", self.0)
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Configuration for a media session
#[derive(Debug, Clone, Default)]
pub struct MediaSessionConfig {
    /// Enable hardware acceleration
    pub hardware_accel: bool,
    /// Maximum buffer size in bytes
    pub max_buffer_size: Option<usize>,
    /// Enable low latency mode
    pub low_latency: bool,
    /// Preferred video decoder
    pub preferred_video_decoder: Option<String>,
    /// Preferred audio decoder
    pub preferred_audio_decoder: Option<String>,
}

impl MediaSessionConfig {
    /// Creates a new default configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables hardware acceleration
    pub fn with_hardware_accel(mut self, enabled: bool) -> Self {
        self.hardware_accel = enabled;
        self
    }

    /// Sets the maximum buffer size
    pub fn with_max_buffer_size(mut self, size: usize) -> Self {
        self.max_buffer_size = Some(size);
        self
    }

    /// Enables low latency mode
    pub fn with_low_latency(mut self, enabled: bool) -> Self {
        self.low_latency = enabled;
        self
    }
}
