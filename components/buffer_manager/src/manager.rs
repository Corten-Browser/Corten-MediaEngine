//! Buffer manager for coordinating resources
//!
//! Coordinates memory allocation and tracks resource usage.

use crate::{BufferConfig, BufferError};

/// Video frame buffer wrapper
///
/// Represents an allocated video frame buffer with automatic cleanup.
#[derive(Debug, PartialEq)]
pub struct VideoFrameBuffer {
    /// Buffer data
    pub data: Vec<u8>,
    /// Buffer size in bytes
    pub size: usize,
}

/// Audio sample buffer wrapper
///
/// Represents an allocated audio sample buffer with automatic cleanup.
#[derive(Debug, PartialEq)]
pub struct AudioSampleBuffer {
    /// Sample data
    pub samples: Vec<f32>,
    /// Number of samples
    pub count: usize,
}

/// Manages buffer resources and memory limits
///
/// Tracks memory usage and enforces limits across all buffer types.
///
/// # Examples
///
/// ```
/// use cortenbrowser_buffer_manager::{BufferManager, BufferConfig};
///
/// let config = BufferConfig::default();
/// let mut manager = BufferManager::new(config);
///
/// let video_buf = manager.allocate_video_buffer(1920 * 1080).unwrap();
/// assert_eq!(video_buf.size, 1920 * 1080);
/// ```
#[derive(Debug)]
pub struct BufferManager {
    config: BufferConfig,
    current_memory: usize,
}

impl BufferManager {
    /// Creates a new buffer manager with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration specifying memory limits
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::{BufferManager, BufferConfig};
    ///
    /// let config = BufferConfig::default();
    /// let manager = BufferManager::new(config);
    /// ```
    pub fn new(config: BufferConfig) -> Self {
        Self {
            config,
            current_memory: 0,
        }
    }

    /// Allocates a video frame buffer
    ///
    /// # Arguments
    ///
    /// * `size` - Size of the buffer in bytes
    ///
    /// # Errors
    ///
    /// Returns `BufferError::OutOfMemory` if allocation would exceed memory limit
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::{BufferManager, BufferConfig};
    ///
    /// let config = BufferConfig::default();
    /// let mut manager = BufferManager::new(config);
    ///
    /// let buffer = manager.allocate_video_buffer(1024).unwrap();
    /// assert_eq!(buffer.size, 1024);
    /// ```
    pub fn allocate_video_buffer(&mut self, size: usize) -> Result<VideoFrameBuffer, BufferError> {
        if self.current_memory + size > self.config.max_memory {
            return Err(BufferError::OutOfMemory);
        }

        self.current_memory += size;

        Ok(VideoFrameBuffer {
            data: vec![0; size],
            size,
        })
    }

    /// Allocates an audio sample buffer
    ///
    /// # Arguments
    ///
    /// * `samples` - Number of f32 samples to allocate
    ///
    /// # Errors
    ///
    /// Returns `BufferError::OutOfMemory` if allocation would exceed memory limit
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::{BufferManager, BufferConfig};
    ///
    /// let config = BufferConfig::default();
    /// let mut manager = BufferManager::new(config);
    ///
    /// let buffer = manager.allocate_audio_buffer(4800).unwrap();
    /// assert_eq!(buffer.count, 4800);
    /// ```
    pub fn allocate_audio_buffer(&mut self, samples: usize) -> Result<AudioSampleBuffer, BufferError> {
        let size = samples * std::mem::size_of::<f32>();

        if self.current_memory + size > self.config.max_memory {
            return Err(BufferError::OutOfMemory);
        }

        self.current_memory += size;

        Ok(AudioSampleBuffer {
            samples: vec![0.0; samples],
            count: samples,
        })
    }

    /// Returns current memory usage in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::{BufferManager, BufferConfig};
    ///
    /// let config = BufferConfig::default();
    /// let mut manager = BufferManager::new(config);
    ///
    /// assert_eq!(manager.get_memory_usage(), 0);
    ///
    /// manager.allocate_video_buffer(1024).unwrap();
    /// assert_eq!(manager.get_memory_usage(), 1024);
    /// ```
    pub fn get_memory_usage(&self) -> usize {
        self.current_memory
    }

    /// Cleans up unused memory
    ///
    /// Returns the amount of memory freed
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::{BufferManager, BufferConfig};
    ///
    /// let config = BufferConfig::default();
    /// let mut manager = BufferManager::new(config);
    ///
    /// let freed = manager.cleanup();
    /// assert_eq!(freed, 0);
    /// ```
    pub fn cleanup(&mut self) -> usize {
        // For now, cleanup doesn't free anything since we don't track allocations
        // In a real implementation, this would free unused buffers
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_manager_has_zero_usage() {
        let config = BufferConfig::default();
        let manager = BufferManager::new(config);
        assert_eq!(manager.get_memory_usage(), 0);
    }

    #[test]
    fn test_allocate_video_buffer() {
        let config = BufferConfig::default();
        let mut manager = BufferManager::new(config);

        let buffer = manager.allocate_video_buffer(1024).unwrap();
        assert_eq!(buffer.size, 1024);
        assert_eq!(buffer.data.len(), 1024);
        assert_eq!(manager.get_memory_usage(), 1024);
    }

    #[test]
    fn test_allocate_audio_buffer() {
        let config = BufferConfig::default();
        let mut manager = BufferManager::new(config);

        let buffer = manager.allocate_audio_buffer(4800).unwrap();
        assert_eq!(buffer.count, 4800);
        assert_eq!(buffer.samples.len(), 4800);

        let expected_size = 4800 * std::mem::size_of::<f32>();
        assert_eq!(manager.get_memory_usage(), expected_size);
    }

    #[test]
    fn test_memory_limit_enforced() {
        let config = BufferConfig {
            max_memory: 2048,
            max_video_frames: 10,
            max_audio_buffers: 10,
        };
        let mut manager = BufferManager::new(config);

        // Allocate within limit
        manager.allocate_video_buffer(1024).unwrap();

        // Try to exceed limit
        let result = manager.allocate_video_buffer(2000);
        assert_eq!(result, Err(BufferError::OutOfMemory));
    }

    #[test]
    fn test_multiple_allocations() {
        let config = BufferConfig::default();
        let mut manager = BufferManager::new(config);

        manager.allocate_video_buffer(512).unwrap();
        manager.allocate_audio_buffer(1000).unwrap();
        manager.allocate_video_buffer(256).unwrap();

        let expected = 512 + (1000 * std::mem::size_of::<f32>()) + 256;
        assert_eq!(manager.get_memory_usage(), expected);
    }

    #[test]
    fn test_cleanup_returns_zero_initially() {
        let config = BufferConfig::default();
        let mut manager = BufferManager::new(config);

        let freed = manager.cleanup();
        assert_eq!(freed, 0);
    }
}
