//! Type definitions for the media pipeline

use std::time::Duration;

/// Configuration for the media pipeline
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineConfig {
    /// Size of internal buffers
    pub buffer_size: usize,
    /// Number of threads for decode operations
    pub thread_count: usize,
    /// Synchronization threshold for A/V sync
    pub sync_threshold: Duration,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            buffer_size: 1024,
            thread_count: 4,
            sync_threshold: Duration::from_millis(40), // 40ms tolerance
        }
    }
}

/// Decision made by the A/V sync controller
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncDecision {
    /// Display the frame immediately
    Display,
    /// Drop the frame (too old)
    Drop,
    /// Wait before displaying
    Wait {
        /// Duration to wait
        duration: Duration,
    },
}
