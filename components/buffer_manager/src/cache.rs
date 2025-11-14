//! Frame cache implementation with LRU eviction
//!
//! Provides an LRU (Least Recently Used) cache for video frames indexed by timestamp.

use cortenbrowser_shared_types::VideoFrame;
use std::time::Duration;
use std::collections::HashMap;
use crate::error::BufferError;

/// Entry in the cache tracking access order
#[derive(Debug, Clone)]
struct CacheEntry {
    frame: VideoFrame,
    access_count: u64,
}

/// LRU cache for video frames
///
/// Stores video frames indexed by timestamp with automatic eviction
/// of least-recently-used frames when capacity is reached.
///
/// # Examples
///
/// ```
/// use cortenbrowser_buffer_manager::FrameCache;
/// use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
/// use std::time::Duration;
///
/// let mut cache = FrameCache::new(10);
///
/// let frame = VideoFrame {
///     width: 1920,
///     height: 1080,
///     format: PixelFormat::YUV420,
///     data: vec![0u8; 100],
///     timestamp: Duration::from_secs(1),
///     duration: Some(Duration::from_millis(33)),
///     metadata: FrameMetadata::default(),
/// };
///
/// cache.insert(frame.clone()).unwrap();
/// let retrieved = cache.get(Duration::from_secs(1));
/// assert!(retrieved.is_some());
/// ```
#[derive(Debug)]
pub struct FrameCache {
    frames: HashMap<Duration, CacheEntry>,
    max_frames: usize,
    access_counter: u64,
}

impl FrameCache {
    /// Creates a new frame cache with the specified maximum number of frames
    ///
    /// # Arguments
    ///
    /// * `max_frames` - Maximum number of frames to cache
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::FrameCache;
    ///
    /// let cache = FrameCache::new(100);
    /// ```
    pub fn new(max_frames: usize) -> Self {
        Self {
            frames: HashMap::new(),
            max_frames,
            access_counter: 0,
        }
    }

    /// Inserts a frame into the cache
    ///
    /// If the cache is full, evicts the least-recently-used frame.
    ///
    /// # Arguments
    ///
    /// * `frame` - The video frame to insert
    ///
    /// # Errors
    ///
    /// Returns `BufferError::OutOfMemory` if max_frames is 0
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::FrameCache;
    /// use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
    /// use std::time::Duration;
    ///
    /// let mut cache = FrameCache::new(10);
    /// let frame = VideoFrame {
    ///     width: 1920,
    ///     height: 1080,
    ///     format: PixelFormat::YUV420,
    ///     data: vec![0u8; 100],
    ///     timestamp: Duration::from_secs(1),
    ///     duration: Some(Duration::from_millis(33)),
    ///     metadata: FrameMetadata::default(),
    /// };
    ///
    /// cache.insert(frame).unwrap();
    /// ```
    pub fn insert(&mut self, frame: VideoFrame) -> Result<(), BufferError> {
        if self.max_frames == 0 {
            return Err(BufferError::OutOfMemory);
        }

        let timestamp = frame.timestamp;

        // If cache is full and this is a new frame, evict LRU
        if self.frames.len() >= self.max_frames && !self.frames.contains_key(&timestamp) {
            // Find and remove least recently used frame
            if let Some((&lru_timestamp, _)) = self.frames.iter()
                .min_by_key(|(_, entry)| entry.access_count) {
                self.frames.remove(&lru_timestamp);
            }
        }

        // Insert or update the frame
        self.access_counter += 1;
        self.frames.insert(timestamp, CacheEntry {
            frame,
            access_count: self.access_counter,
        });

        Ok(())
    }

    /// Gets a frame by timestamp
    ///
    /// Updates the access count for LRU tracking.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The timestamp of the frame to retrieve
    ///
    /// # Returns
    ///
    /// Some(frame) if found, None otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::FrameCache;
    /// use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
    /// use std::time::Duration;
    ///
    /// let mut cache = FrameCache::new(10);
    /// let frame = VideoFrame {
    ///     width: 1920,
    ///     height: 1080,
    ///     format: PixelFormat::YUV420,
    ///     data: vec![0u8; 100],
    ///     timestamp: Duration::from_secs(1),
    ///     duration: Some(Duration::from_millis(33)),
    ///     metadata: FrameMetadata::default(),
    /// };
    ///
    /// cache.insert(frame).unwrap();
    /// let retrieved = cache.get(Duration::from_secs(1));
    /// assert!(retrieved.is_some());
    /// ```
    pub fn get(&mut self, timestamp: Duration) -> Option<VideoFrame> {
        if let Some(entry) = self.frames.get_mut(&timestamp) {
            // Update access count for LRU tracking
            self.access_counter += 1;
            entry.access_count = self.access_counter;
            Some(entry.frame.clone())
        } else {
            None
        }
    }

    /// Evicts frames before the given timestamp
    ///
    /// Useful for removing old frames that are no longer needed.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Remove all frames with timestamps before this value
    ///
    /// # Returns
    ///
    /// The number of frames evicted
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::FrameCache;
    /// use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
    /// use std::time::Duration;
    ///
    /// let mut cache = FrameCache::new(10);
    ///
    /// for i in 0..5 {
    ///     let frame = VideoFrame {
    ///         width: 1920,
    ///         height: 1080,
    ///         format: PixelFormat::YUV420,
    ///         data: vec![0u8; 100],
    ///         timestamp: Duration::from_secs(i),
    ///         duration: Some(Duration::from_millis(33)),
    ///         metadata: FrameMetadata::default(),
    ///     };
    ///     cache.insert(frame).unwrap();
    /// }
    ///
    /// let evicted = cache.evict_before(Duration::from_secs(3));
    /// assert_eq!(evicted, 3);
    /// ```
    pub fn evict_before(&mut self, timestamp: Duration) -> usize {
        let to_remove: Vec<Duration> = self.frames.keys()
            .filter(|&&ts| ts < timestamp)
            .copied()
            .collect();

        let count = to_remove.len();
        for ts in to_remove {
            self.frames.remove(&ts);
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cortenbrowser_shared_types::{PixelFormat, FrameMetadata};

    fn create_test_frame(timestamp_secs: u64) -> VideoFrame {
        VideoFrame {
            width: 1920,
            height: 1080,
            format: PixelFormat::YUV420,
            data: vec![0u8; 100],
            timestamp: Duration::from_secs(timestamp_secs),
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata::default(),
        }
    }

    #[test]
    fn test_new_cache_is_empty() {
        let mut cache = FrameCache::new(10);
        let result = cache.get(Duration::from_secs(0));
        assert!(result.is_none());
    }

    #[test]
    fn test_insert_and_get_frame() {
        let mut cache = FrameCache::new(10);
        let frame = create_test_frame(1);

        cache.insert(frame.clone()).unwrap();
        let retrieved = cache.get(Duration::from_secs(1));

        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.timestamp, Duration::from_secs(1));
        assert_eq!(retrieved.width, 1920);
    }

    #[test]
    fn test_get_nonexistent_frame() {
        let mut cache = FrameCache::new(10);
        let result = cache.get(Duration::from_secs(99));
        assert!(result.is_none());
    }

    #[test]
    fn test_lru_eviction_when_full() {
        let mut cache = FrameCache::new(3);

        // Insert 3 frames
        cache.insert(create_test_frame(1)).unwrap();
        cache.insert(create_test_frame(2)).unwrap();
        cache.insert(create_test_frame(3)).unwrap();

        // Access frame 1 to make it recently used
        cache.get(Duration::from_secs(1));

        // Insert 4th frame - should evict frame 2 (least recently used)
        cache.insert(create_test_frame(4)).unwrap();

        assert!(cache.get(Duration::from_secs(1)).is_some());
        assert!(cache.get(Duration::from_secs(2)).is_none()); // Evicted
        assert!(cache.get(Duration::from_secs(3)).is_some());
        assert!(cache.get(Duration::from_secs(4)).is_some());
    }

    #[test]
    fn test_evict_before_timestamp() {
        let mut cache = FrameCache::new(10);

        // Insert frames at different timestamps
        for i in 0..5 {
            cache.insert(create_test_frame(i)).unwrap();
        }

        // Evict frames before t=3
        let evicted = cache.evict_before(Duration::from_secs(3));
        assert_eq!(evicted, 3);

        // Check remaining frames
        assert!(cache.get(Duration::from_secs(0)).is_none());
        assert!(cache.get(Duration::from_secs(1)).is_none());
        assert!(cache.get(Duration::from_secs(2)).is_none());
        assert!(cache.get(Duration::from_secs(3)).is_some());
        assert!(cache.get(Duration::from_secs(4)).is_some());
    }

    #[test]
    fn test_update_existing_frame() {
        let mut cache = FrameCache::new(10);

        let frame1 = create_test_frame(1);
        cache.insert(frame1).unwrap();

        // Insert again with same timestamp - should update
        let mut frame2 = create_test_frame(1);
        frame2.width = 3840; // Different width

        cache.insert(frame2).unwrap();

        let retrieved = cache.get(Duration::from_secs(1)).unwrap();
        assert_eq!(retrieved.width, 3840);
    }

    #[test]
    fn test_cache_with_zero_capacity() {
        let mut cache = FrameCache::new(0);
        let frame = create_test_frame(1);

        let result = cache.insert(frame);
        assert_eq!(result, Err(BufferError::OutOfMemory));
    }
}
