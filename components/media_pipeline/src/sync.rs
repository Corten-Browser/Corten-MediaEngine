//! Audio/Video synchronization controller
//!
//! Manages synchronization between audio and video streams to ensure
//! they play in sync with minimal drift.

use crate::types::SyncDecision;
use cortenbrowser_shared_types::VideoFrame;
use parking_lot::RwLock;
use std::time::Duration;

/// Default synchronization threshold (40ms)
const DEFAULT_SYNC_THRESHOLD: Duration = Duration::from_millis(40);

/// A/V synchronization controller
///
/// The AVSyncController maintains a media clock and makes decisions about
/// whether video frames should be displayed, dropped, or delayed based on
/// their timestamp relative to the audio stream.
///
/// # Examples
///
/// ```
/// use cortenbrowser_media_pipeline::AVSyncController;
/// use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
/// use std::time::Duration;
///
/// let controller = AVSyncController::new();
///
/// let frame = VideoFrame {
///     width: 1920,
///     height: 1080,
///     format: PixelFormat::YUV420,
///     data: vec![0u8; 1920 * 1080],
///     timestamp: Duration::from_secs(1),
///     duration: Some(Duration::from_millis(33)),
///     metadata: FrameMetadata::default(),
/// };
///
/// let audio_timestamp = Duration::from_secs(1);
/// let decision = controller.sync_frame(&frame, audio_timestamp);
/// ```
#[derive(Debug)]
pub struct AVSyncController {
    /// Current media clock position
    clock: RwLock<Duration>,
    /// Synchronization threshold
    threshold: Duration,
}

impl AVSyncController {
    /// Creates a new A/V sync controller
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_pipeline::AVSyncController;
    ///
    /// let controller = AVSyncController::new();
    /// ```
    pub fn new() -> Self {
        Self {
            clock: RwLock::new(Duration::ZERO),
            threshold: DEFAULT_SYNC_THRESHOLD,
        }
    }

    /// Makes a synchronization decision for a video frame
    ///
    /// Compares the video frame timestamp with the current audio timestamp
    /// and decides whether to display, drop, or wait.
    ///
    /// # Arguments
    ///
    /// * `video_frame` - The video frame to synchronize
    /// * `audio_timestamp` - Current audio playback timestamp
    ///
    /// # Returns
    ///
    /// A `SyncDecision` indicating what to do with the frame
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_pipeline::{AVSyncController, SyncDecision};
    /// use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
    /// use std::time::Duration;
    ///
    /// let controller = AVSyncController::new();
    ///
    /// let frame = VideoFrame {
    ///     width: 1920,
    ///     height: 1080,
    ///     format: PixelFormat::YUV420,
    ///     data: vec![0u8; 1920 * 1080],
    ///     timestamp: Duration::from_millis(1000),
    ///     duration: Some(Duration::from_millis(33)),
    ///     metadata: FrameMetadata::default(),
    /// };
    ///
    /// let decision = controller.sync_frame(&frame, Duration::from_millis(1000));
    /// assert_eq!(decision, SyncDecision::Display);
    /// ```
    pub fn sync_frame(&self, video_frame: &VideoFrame, audio_timestamp: Duration) -> SyncDecision {
        let video_timestamp = video_frame.timestamp;

        // Calculate time difference (positive if video is ahead, negative if behind)
        let diff = if video_timestamp >= audio_timestamp {
            video_timestamp - audio_timestamp
        } else {
            // Video is behind audio
            let behind_by = audio_timestamp - video_timestamp;

            // If video is significantly behind (more than threshold), drop the frame
            if behind_by > self.threshold {
                return SyncDecision::Drop;
            }

            // Within threshold, display it
            return SyncDecision::Display;
        };

        // Video is ahead of audio
        if diff <= self.threshold {
            // Within tolerance, display immediately
            self.update_clock(video_timestamp);
            SyncDecision::Display
        } else {
            // Too far ahead, need to wait
            SyncDecision::Wait { duration: diff }
        }
    }

    /// Gets the current media clock position
    ///
    /// # Returns
    ///
    /// The current clock time as a `Duration`
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_pipeline::AVSyncController;
    /// use std::time::Duration;
    ///
    /// let controller = AVSyncController::new();
    /// assert_eq!(controller.get_clock(), Duration::ZERO);
    /// ```
    pub fn get_clock(&self) -> Duration {
        *self.clock.read()
    }

    /// Updates the internal clock to the given timestamp
    fn update_clock(&self, timestamp: Duration) {
        let mut clock = self.clock.write();
        if timestamp > *clock {
            *clock = timestamp;
        }
    }
}

impl Default for AVSyncController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cortenbrowser_shared_types::{FrameMetadata, PixelFormat};

    fn create_test_frame(timestamp: Duration) -> VideoFrame {
        VideoFrame {
            width: 1920,
            height: 1080,
            format: PixelFormat::YUV420,
            data: vec![0u8; 1920 * 1080],
            timestamp,
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata::default(),
        }
    }

    #[test]
    fn test_new_controller_has_zero_clock() {
        let controller = AVSyncController::new();
        assert_eq!(controller.get_clock(), Duration::ZERO);
    }

    #[test]
    fn test_sync_in_sync_frames() {
        let controller = AVSyncController::new();
        let frame = create_test_frame(Duration::from_secs(1));
        let decision = controller.sync_frame(&frame, Duration::from_secs(1));
        assert_eq!(decision, SyncDecision::Display);
    }

    #[test]
    fn test_drop_late_frames() {
        let controller = AVSyncController::new();
        // Frame is 100ms behind (more than 40ms threshold)
        let frame = create_test_frame(Duration::from_millis(900));
        let decision = controller.sync_frame(&frame, Duration::from_millis(1000));
        assert_eq!(decision, SyncDecision::Drop);
    }

    #[test]
    fn test_wait_for_early_frames() {
        let controller = AVSyncController::new();
        // Frame is 50ms ahead (more than 40ms threshold)
        let frame = create_test_frame(Duration::from_millis(1050));
        let decision = controller.sync_frame(&frame, Duration::from_millis(1000));

        match decision {
            SyncDecision::Wait { duration } => {
                assert!(duration > Duration::ZERO);
            }
            _ => panic!("Expected Wait decision"),
        }
    }

    #[test]
    fn test_display_slightly_behind_frames() {
        let controller = AVSyncController::new();
        // Frame is 20ms behind (within 40ms threshold)
        let frame = create_test_frame(Duration::from_millis(980));
        let decision = controller.sync_frame(&frame, Duration::from_millis(1000));
        assert_eq!(decision, SyncDecision::Display);
    }
}
