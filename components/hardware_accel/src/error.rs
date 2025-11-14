//! Error types for hardware acceleration operations

use thiserror::Error;

/// Hardware acceleration error types
///
/// # Examples
///
/// ```
/// use cortenbrowser_hardware_accel::HardwareError;
///
/// let error = HardwareError::NotAvailable;
/// println!("Error: {}", error);
/// ```
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum HardwareError {
    /// Hardware acceleration is not available on this platform
    #[error("Hardware acceleration not available")]
    NotAvailable,

    /// The requested codec is not supported by hardware
    #[error("Codec not supported by hardware")]
    UnsupportedCodec,

    /// Hardware decoder initialization failed
    #[error("Hardware decoder initialization failed")]
    InitializationFailed,

    /// Hardware decode operation failed
    #[error("Hardware decode operation failed")]
    DecodeFailed,
}

/// Result type for hardware acceleration operations
pub type HardwareResult<T> = Result<T, HardwareError>;
