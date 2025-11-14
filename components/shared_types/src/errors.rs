//! Error types for media engine operations
//!
//! This module defines all error types that can occur during media processing.

use thiserror::Error;

/// Session state for state transition errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    /// Session is idle
    Idle,
    /// Session is loading media
    Loading,
    /// Session is ready to play
    Ready,
    /// Session is playing
    Playing,
    /// Session is paused
    Paused,
    /// Session is seeking
    Seeking,
    /// Session has ended
    Ended,
    /// Session encountered an error
    Error,
}

/// Media engine error types
///
/// # Examples
///
/// ```
/// use cortenbrowser_shared_types::MediaError;
///
/// let error = MediaError::UnsupportedFormat {
///     format: "FLV".to_string(),
/// };
/// println!("Error: {}", error);
/// ```
#[derive(Debug, Clone, Error, PartialEq)]
pub enum MediaError {
    /// The media format is not supported
    #[error("Unsupported format: {format}")]
    UnsupportedFormat {
        /// The unsupported format identifier
        format: String,
    },

    /// An error occurred during codec operations
    #[error("Codec error: {details}")]
    CodecError {
        /// Details about the codec error
        details: String,
    },

    /// A network error occurred while loading media
    #[error("Network error: {details}")]
    NetworkError {
        /// Details about the network error
        details: String,
    },

    /// A DRM/encryption error occurred
    #[error("DRM error: {details}")]
    DrmError {
        /// Details about the DRM error
        details: String,
    },

    /// A hardware acceleration error occurred
    #[error("Hardware error: {details}")]
    HardwareError {
        /// Details about the hardware error
        details: String,
    },

    /// The system ran out of memory
    #[error("Out of memory")]
    OutOfMemory,

    /// An invalid state transition was attempted
    #[error("Invalid state transition: {from:?} -> {to:?}")]
    InvalidStateTransition {
        /// The current state
        from: SessionState,
        /// The attempted target state
        to: SessionState,
    },
}

/// Result type for media operations
pub type MediaResult<T> = Result<T, MediaError>;
