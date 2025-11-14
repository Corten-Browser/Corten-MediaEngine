//! Configuration types for buffer management

/// Configuration for buffer manager
#[derive(Debug, Clone)]
pub struct BufferConfig {
    /// Maximum memory usage in bytes
    pub max_memory: usize,
    /// Maximum number of video frames to cache
    pub max_video_frames: usize,
    /// Maximum number of audio buffers
    pub max_audio_buffers: usize,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            // Default to 100MB max memory
            max_memory: 100 * 1024 * 1024,
            // Default to 100 frames
            max_video_frames: 100,
            // Default to 50 audio buffers
            max_audio_buffers: 50,
        }
    }
}
