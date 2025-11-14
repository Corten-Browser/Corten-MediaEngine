//! Media pipeline orchestration
//!
//! Coordinates source readers, demuxers, decoders, and synchronization.

use crate::types::PipelineConfig;
use crate::AVSyncController;
use cortenbrowser_shared_types::{AudioBuffer, MediaError, MediaSource, VideoFrame};
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

/// Pipeline state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipelineState {
    /// Pipeline is idle (no source loaded)
    Idle,
    /// Pipeline is loading a source
    Loading,
    /// Pipeline is ready to play (source loaded)
    Ready,
    /// Pipeline is running (actively processing)
    Running,
    /// Pipeline is stopped
    Stopped,
}

/// Main media pipeline orchestrator
///
/// The MediaPipeline coordinates source readers, demuxers, decoders, and
/// audio/video synchronization. It manages the complete media processing flow.
///
/// # Examples
///
/// ```
/// use cortenbrowser_media_pipeline::{MediaPipeline, PipelineConfig};
/// use cortenbrowser_shared_types::MediaSource;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = PipelineConfig::default();
/// let pipeline = MediaPipeline::new(config)?;
///
/// let source = MediaSource::Url {
///     url: "file:///test/video.mp4".to_string(),
/// };
///
/// pipeline.load_source(source).await?;
/// pipeline.start().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct MediaPipeline {
    /// Pipeline configuration
    config: PipelineConfig,
    /// Current pipeline state
    state: Arc<RwLock<PipelineState>>,
    /// A/V sync controller
    sync_controller: Arc<AVSyncController>,
    /// Currently loaded media source
    source: Arc<RwLock<Option<MediaSource>>>,
    /// Video frame queue (sender)
    video_tx: mpsc::Sender<VideoFrame>,
    /// Video frame queue (receiver)
    video_rx: Arc<RwLock<Option<mpsc::Receiver<VideoFrame>>>>,
    /// Audio buffer queue (sender)
    audio_tx: mpsc::Sender<AudioBuffer>,
    /// Audio buffer queue (receiver)
    audio_rx: Arc<RwLock<Option<mpsc::Receiver<AudioBuffer>>>>,
}

impl MediaPipeline {
    /// Creates a new media pipeline with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Pipeline configuration
    ///
    /// # Returns
    ///
    /// A new `MediaPipeline` instance or an error
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_pipeline::{MediaPipeline, PipelineConfig};
    ///
    /// let config = PipelineConfig::default();
    /// let pipeline = MediaPipeline::new(config).unwrap();
    /// ```
    pub fn new(config: PipelineConfig) -> Result<Self, MediaError> {
        let buffer_size = config.buffer_size;

        // Create video frame queue
        let (video_tx, video_rx) = mpsc::channel(buffer_size);

        // Create audio buffer queue
        let (audio_tx, audio_rx) = mpsc::channel(buffer_size);

        Ok(Self {
            config,
            state: Arc::new(RwLock::new(PipelineState::Idle)),
            sync_controller: Arc::new(AVSyncController::new()),
            source: Arc::new(RwLock::new(None)),
            video_tx,
            video_rx: Arc::new(RwLock::new(Some(video_rx))),
            audio_tx,
            audio_rx: Arc::new(RwLock::new(Some(audio_rx))),
        })
    }

    /// Loads a media source into the pipeline
    ///
    /// # Arguments
    ///
    /// * `source` - The media source to load
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_pipeline::{MediaPipeline, PipelineConfig};
    /// use cortenbrowser_shared_types::MediaSource;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let pipeline = MediaPipeline::new(PipelineConfig::default())?;
    ///
    /// let source = MediaSource::Url {
    ///     url: "file:///test/video.mp4".to_string(),
    /// };
    ///
    /// pipeline.load_source(source).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load_source(&self, source: MediaSource) -> Result<(), MediaError> {
        let mut state = self.state.write();

        // Can only load in Idle or Stopped states
        if *state != PipelineState::Idle && *state != PipelineState::Stopped {
            return Err(MediaError::InvalidStateTransition {
                from: cortenbrowser_shared_types::SessionState::Loading,
                to: cortenbrowser_shared_types::SessionState::Ready,
            });
        }

        *state = PipelineState::Loading;
        drop(state); // Release lock

        // Store the source
        {
            let mut src = self.source.write();
            *src = Some(source);
        }

        // Transition to Ready state
        {
            let mut state = self.state.write();
            *state = PipelineState::Ready;
        }

        Ok(())
    }

    /// Starts the pipeline (begins processing)
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_pipeline::{MediaPipeline, PipelineConfig};
    /// use cortenbrowser_shared_types::MediaSource;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let pipeline = MediaPipeline::new(PipelineConfig::default())?;
    ///
    /// let source = MediaSource::Url {
    ///     url: "file:///test/video.mp4".to_string(),
    /// };
    ///
    /// pipeline.load_source(source).await?;
    /// pipeline.start().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start(&self) -> Result<(), MediaError> {
        let mut state = self.state.write();

        // Can only start from Ready state
        if *state != PipelineState::Ready {
            return Err(MediaError::InvalidStateTransition {
                from: cortenbrowser_shared_types::SessionState::Idle,
                to: cortenbrowser_shared_types::SessionState::Playing,
            });
        }

        *state = PipelineState::Running;

        // TODO: Actually start demuxing/decoding threads
        // This would spawn worker tasks for:
        // - Source reading
        // - Demuxing
        // - Video decoding
        // - Audio decoding

        Ok(())
    }

    /// Stops the pipeline
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_pipeline::{MediaPipeline, PipelineConfig};
    /// use cortenbrowser_shared_types::MediaSource;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let pipeline = MediaPipeline::new(PipelineConfig::default())?;
    ///
    /// let source = MediaSource::Url {
    ///     url: "file:///test/video.mp4".to_string(),
    /// };
    ///
    /// pipeline.load_source(source).await?;
    /// pipeline.start().await?;
    /// pipeline.stop().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stop(&self) -> Result<(), MediaError> {
        let mut state = self.state.write();

        // Can stop from Running state
        if *state != PipelineState::Running {
            return Err(MediaError::InvalidStateTransition {
                from: cortenbrowser_shared_types::SessionState::Idle,
                to: cortenbrowser_shared_types::SessionState::Paused,
            });
        }

        *state = PipelineState::Stopped;

        // TODO: Actually stop worker threads
        // This would cancel all worker tasks

        Ok(())
    }

    /// Seeks to a specific position in the media
    ///
    /// # Arguments
    ///
    /// * `position` - Target seek position
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_pipeline::{MediaPipeline, PipelineConfig};
    /// use cortenbrowser_shared_types::MediaSource;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let pipeline = MediaPipeline::new(PipelineConfig::default())?;
    ///
    /// let source = MediaSource::Url {
    ///     url: "file:///test/video.mp4".to_string(),
    /// };
    ///
    /// pipeline.load_source(source).await?;
    /// pipeline.start().await?;
    /// pipeline.seek(Duration::from_secs(10)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn seek(&self, _position: Duration) -> Result<(), MediaError> {
        let state = self.state.read();

        // Can only seek in Running or Ready states
        if *state != PipelineState::Running && *state != PipelineState::Ready {
            return Err(MediaError::InvalidStateTransition {
                from: cortenbrowser_shared_types::SessionState::Idle,
                to: cortenbrowser_shared_types::SessionState::Seeking,
            });
        }

        // TODO: Actually seek in the media
        // This would:
        // - Flush buffers
        // - Seek in the demuxer
        // - Reset decoder state
        // - Seek to nearest keyframe

        Ok(())
    }

    /// Gets the next video frame from the pipeline
    ///
    /// # Returns
    ///
    /// The next video frame, or `None` if no frame is available
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_pipeline::{MediaPipeline, PipelineConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let pipeline = MediaPipeline::new(PipelineConfig::default())?;
    ///
    /// let frame = pipeline.get_next_video_frame().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_next_video_frame(&self) -> Option<VideoFrame> {
        let mut rx_guard = self.video_rx.write();

        if let Some(rx) = rx_guard.as_mut() {
            rx.try_recv().ok()
        } else {
            None
        }
    }

    /// Gets the next audio buffer from the pipeline
    ///
    /// # Returns
    ///
    /// The next audio buffer, or `None` if no buffer is available
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_pipeline::{MediaPipeline, PipelineConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let pipeline = MediaPipeline::new(PipelineConfig::default())?;
    ///
    /// let buffer = pipeline.get_next_audio_buffer().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_next_audio_buffer(&self) -> Option<AudioBuffer> {
        let mut rx_guard = self.audio_rx.write();

        if let Some(rx) = rx_guard.as_mut() {
            rx.try_recv().ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_pipeline() {
        let config = PipelineConfig::default();
        let result = MediaPipeline::new(config);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_state_transitions() {
        let pipeline = MediaPipeline::new(PipelineConfig::default()).unwrap();

        // Initial state should be Idle
        assert_eq!(*pipeline.state.read(), PipelineState::Idle);

        // Load source
        let source = MediaSource::Url {
            url: "file:///test.mp4".to_string(),
        };
        pipeline.load_source(source).await.unwrap();

        // State should be Ready
        assert_eq!(*pipeline.state.read(), PipelineState::Ready);

        // Start pipeline
        pipeline.start().await.unwrap();

        // State should be Running
        assert_eq!(*pipeline.state.read(), PipelineState::Running);

        // Stop pipeline
        pipeline.stop().await.unwrap();

        // State should be Stopped
        assert_eq!(*pipeline.state.read(), PipelineState::Stopped);
    }

    #[tokio::test]
    async fn test_invalid_state_transition() {
        let pipeline = MediaPipeline::new(PipelineConfig::default()).unwrap();

        // Try to start without loading source
        let result = pipeline.start().await;
        assert!(result.is_err());
    }
}
