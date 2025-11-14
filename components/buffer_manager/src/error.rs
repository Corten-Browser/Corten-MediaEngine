//! Error types for buffer management

use thiserror::Error;

/// Errors that can occur during buffer operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BufferError {
    /// Buffer is out of memory
    #[error("Out of memory")]
    OutOfMemory,

    /// Buffer is full and cannot accept more data
    #[error("Buffer is full")]
    BufferFull,

    /// Buffer is empty and cannot provide data
    #[error("Buffer is empty")]
    BufferEmpty,

    /// Invalid size parameter
    #[error("Invalid size: {0}")]
    InvalidSize(String),
}
