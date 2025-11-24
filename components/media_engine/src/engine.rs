///! Media Engine implementation - coordinates all media components
use crate::types::{MediaEngineConfig, MediaEngineEvent, MediaEngineMessage};
use cortenbrowser_media_pipeline::MediaPipeline;
use cortenbrowser_media_session::{MediaSession, SessionManager, SessionState};
use cortenbrowser_shared_types::{
    AudioBuffer, MediaEngine, MediaError, MediaSessionConfig, MediaSource, SessionId, VideoFrame,
};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// Media Engine implementation
///
/// Coordinates all media components including session management, pipeline orchestration,
/// format parsing, decoding, buffering, and synchronization.
pub struct MediaEngineImpl {
    /// Configuration
    config: MediaEngineConfig,
    /// Session manager
    session_manager: Arc<SessionManager>,
    /// Active sessions with their pipelines
    sessions: Arc<RwLock<HashMap<SessionId, SessionContext>>>,
    /// Message receiver channel
    message_rx: Arc<RwLock<Option<mpsc::UnboundedReceiver<MediaEngineMessage>>>>,
    /// Event sender channel
    event_tx: mpsc::UnboundedSender<MediaEngineEvent>,
    /// Event receiver channel (for users of the engine)
    event_rx: Arc<RwLock<Option<mpsc::UnboundedReceiver<MediaEngineEvent>>>>,
}

/// Context for a single media session
struct SessionContext {
    /// The media session
    session: Arc<MediaSession>,
    /// The media pipeline for this session
    pipeline: Option<Arc<MediaPipeline>>,
}

impl MediaEngineImpl {
    /// Create a new Media Engine
    ///
    /// # Arguments
    /// * `config` - Media engine configuration
    ///
    /// # Returns
    /// * `Ok(MediaEngineImpl)` - Successfully created engine
    /// * `Err(MediaError)` - Failed to create engine
    pub fn new(config: MediaEngineConfig) -> Result<Self, MediaError> {
        info!("Creating MediaEngine with config: {:?}", config);

        // Create session manager
        let session_manager = Arc::new(SessionManager::new());

        // Create message/event channels
        let (message_tx, message_rx) = mpsc::unbounded_channel();
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Ok(Self {
            config,
            session_manager,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            message_rx: Arc::new(RwLock::new(Some(message_rx))),
            event_tx,
            event_rx: Arc::new(RwLock::new(Some(event_rx))),
        })
    }

    /// Get the message sender channel
    ///
    /// Users can send MediaEngineMessage through this channel
    pub fn message_sender(&self) -> mpsc::UnboundedSender<MediaEngineMessage> {
        // Create a new sender from the receiver
        // Note: In real implementation, we'd store the sender separately
        unimplemented!("Message sender needs proper implementation")
    }

    /// Take the event receiver channel
    ///
    /// Users can receive MediaEngineEvent through this channel
    pub fn take_event_receiver(&self) -> Option<mpsc::UnboundedReceiver<MediaEngineEvent>> {
        self.event_rx.write().take()
    }

    /// Handle a message
    async fn handle_message(&self, message: MediaEngineMessage) -> Result<(), MediaError> {
        match message {
            MediaEngineMessage::CreateMediaElement {
                element_id,
                attributes,
            } => {
                debug!("Creating media element: {}", element_id);
                // TODO: Implement media element creation
                Ok(())
            }
            MediaEngineMessage::StreamData { session_id, chunk } => {
                debug!("Received stream data for session: {:?}", session_id);
                // TODO: Feed chunk to pipeline
                Ok(())
            }
            MediaEngineMessage::PlaybackCommand {
                session_id,
                command,
            } => {
                debug!(
                    "Playback command for session {:?}: {:?}",
                    session_id, command
                );
                // TODO: Execute playback command
                Ok(())
            }
        }
    }

    /// Emit an event
    fn emit_event(&self, event: MediaEngineEvent) {
        if let Err(e) = self.event_tx.send(event) {
            error!("Failed to send event: {}", e);
        }
    }
}

impl MediaEngine for MediaEngineImpl {
    async fn create_session(&self, config: MediaSessionConfig) -> Result<SessionId, MediaError> {
        info!("Creating media session with config: {:?}", config);

        // Check session limit
        {
            let sessions = self.sessions.read();
            if sessions.len() >= self.config.max_sessions {
                return Err(MediaError::ResourceExhausted(format!(
                    "Maximum sessions ({}) reached",
                    self.config.max_sessions
                )));
            }
        }

        // Create session through session manager
        let session_id = self.session_manager.create(config)?;

        // Get the session
        let session = self
            .session_manager
            .get(session_id)
            .ok_or_else(|| MediaError::SessionNotFound(session_id))?;

        // Store session context (without pipeline initially)
        let context = SessionContext {
            session,
            pipeline: None,
        };

        self.sessions.write().insert(session_id, context);

        info!("Created session: {:?}", session_id);
        Ok(session_id)
    }

    async fn load_source(&self, session: SessionId, source: MediaSource) -> Result<(), MediaError> {
        info!("Loading source for session: {:?}", session);

        // Get session context
        let mut sessions = self.sessions.write();
        let context = sessions
            .get_mut(&session)
            .ok_or_else(|| MediaError::SessionNotFound(session))?;

        // Create pipeline for this session
        let pipeline = MediaPipeline::new(self.config.pipeline_config.clone())?;

        // TODO: Configure pipeline with source
        // pipeline.set_source(source)?;

        context.pipeline = Some(Arc::new(pipeline));

        info!("Loaded source for session: {:?}", session);
        Ok(())
    }

    async fn play(&self, session: SessionId) -> Result<(), MediaError> {
        info!("Play requested for session: {:?}", session);

        let sessions = self.sessions.read();
        let context = sessions
            .get(&session)
            .ok_or_else(|| MediaError::SessionNotFound(session))?;

        // Transition session state
        context.session.set_state(SessionState::Playing {
            position: Duration::from_secs(0),
            rate: 1.0,
        });

        // Start pipeline
        if let Some(pipeline) = &context.pipeline {
            // TODO: Start pipeline playback
            debug!("Starting pipeline for session: {:?}", session);
        }

        // Emit state changed event
        self.emit_event(MediaEngineEvent::PlaybackStateChanged {
            session_id: session,
            state: SessionState::Playing {
                position: Duration::from_secs(0),
                rate: 1.0,
            },
        });

        Ok(())
    }

    async fn pause(&self, session: SessionId) -> Result<(), MediaError> {
        info!("Pause requested for session: {:?}", session);

        let sessions = self.sessions.read();
        let context = sessions
            .get(&session)
            .ok_or_else(|| MediaError::SessionNotFound(session))?;

        // Get current position
        let position = Duration::from_secs(0); // TODO: Get from pipeline

        // Transition session state
        context
            .session
            .set_state(SessionState::Paused { position });

        // Pause pipeline
        if let Some(pipeline) = &context.pipeline {
            // TODO: Pause pipeline
            debug!("Pausing pipeline for session: {:?}", session);
        }

        // Emit state changed event
        self.emit_event(MediaEngineEvent::PlaybackStateChanged {
            session_id: session,
            state: SessionState::Paused { position },
        });

        Ok(())
    }

    async fn seek(&self, session: SessionId, position: Duration) -> Result<(), MediaError> {
        info!(
            "Seek to {:?} requested for session: {:?}",
            position, session
        );

        let sessions = self.sessions.read();
        let context = sessions
            .get(&session)
            .ok_or_else(|| MediaError::SessionNotFound(session))?;

        // Transition to seeking state
        context
            .session
            .set_state(SessionState::Seeking { target: position });

        // Seek in pipeline
        if let Some(pipeline) = &context.pipeline {
            // TODO: Perform seek in pipeline
            debug!(
                "Seeking pipeline to {:?} for session: {:?}",
                position, session
            );
        }

        // Transition back to playing/paused
        context.session.set_state(SessionState::Playing {
            position,
            rate: 1.0,
        });

        // Emit state changed event
        self.emit_event(MediaEngineEvent::PlaybackStateChanged {
            session_id: session,
            state: SessionState::Playing {
                position,
                rate: 1.0,
            },
        });

        Ok(())
    }

    async fn set_volume(&self, session: SessionId, volume: f32) -> Result<(), MediaError> {
        info!("Set volume to {} for session: {:?}", volume, session);

        // Validate volume range
        if !(0.0..=1.0).contains(&volume) {
            return Err(MediaError::InvalidParameter(format!(
                "Volume must be between 0.0 and 1.0, got {}",
                volume
            )));
        }

        let sessions = self.sessions.read();
        let _context = sessions
            .get(&session)
            .ok_or_else(|| MediaError::SessionNotFound(session))?;

        // TODO: Set volume on audio output
        debug!("Setting volume to {} for session: {:?}", volume, session);

        Ok(())
    }

    async fn get_video_frame(&self, session: SessionId) -> Result<VideoFrame, MediaError> {
        debug!("Get video frame for session: {:?}", session);

        let sessions = self.sessions.read();
        let context = sessions
            .get(&session)
            .ok_or_else(|| MediaError::SessionNotFound(session))?;

        // Get frame from pipeline
        if let Some(pipeline) = &context.pipeline {
            // TODO: Get frame from pipeline
            debug!(
                "Getting video frame from pipeline for session: {:?}",
                session
            );
            // For now, return a placeholder error
            return Err(MediaError::NotImplemented(
                "get_video_frame not yet implemented".to_string(),
            ));
        }

        Err(MediaError::InvalidState(
            "No pipeline for session".to_string(),
        ))
    }

    async fn get_audio_samples(
        &self,
        session: SessionId,
        count: usize,
    ) -> Result<AudioBuffer, MediaError> {
        debug!("Get {} audio samples for session: {:?}", count, session);

        let sessions = self.sessions.read();
        let context = sessions
            .get(&session)
            .ok_or_else(|| MediaError::SessionNotFound(session))?;

        // Get samples from pipeline
        if let Some(pipeline) = &context.pipeline {
            // TODO: Get samples from pipeline
            debug!(
                "Getting {} audio samples from pipeline for session: {:?}",
                count, session
            );
            // For now, return a placeholder error
            return Err(MediaError::NotImplemented(
                "get_audio_samples not yet implemented".to_string(),
            ));
        }

        Err(MediaError::InvalidState(
            "No pipeline for session".to_string(),
        ))
    }

    async fn destroy_session(&self, session: SessionId) -> Result<(), MediaError> {
        info!("Destroying session: {:?}", session);

        // Remove session context
        let context = self
            .sessions
            .write()
            .remove(&session)
            .ok_or_else(|| MediaError::SessionNotFound(session))?;

        // Stop pipeline if exists
        if let Some(pipeline) = context.pipeline {
            // TODO: Stop and cleanup pipeline
            debug!("Stopping pipeline for session: {:?}", session);
        }

        // Destroy session through manager
        self.session_manager.destroy(session)?;

        info!("Destroyed session: {:?}", session);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_engine() {
        let config = MediaEngineConfig::default();
        let engine = MediaEngineImpl::new(config);
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_create_session() {
        let config = MediaEngineConfig::default();
        let engine = MediaEngineImpl::new(config).unwrap();

        let session_config = MediaSessionConfig::default();
        let result = engine.create_session(session_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_session_limit() {
        let config = MediaEngineConfig {
            max_sessions: 1,
            ..Default::default()
        };
        let engine = MediaEngineImpl::new(config).unwrap();

        // Create first session - should succeed
        let session1 = engine.create_session(MediaSessionConfig::default()).await;
        assert!(session1.is_ok());

        // Create second session - should fail due to limit
        let session2 = engine.create_session(MediaSessionConfig::default()).await;
        assert!(session2.is_err());
    }

    #[tokio::test]
    async fn test_play_pause() {
        let config = MediaEngineConfig::default();
        let engine = MediaEngineImpl::new(config).unwrap();

        let session = engine
            .create_session(MediaSessionConfig::default())
            .await
            .unwrap();

        // Play should succeed
        let result = engine.play(session).await;
        assert!(result.is_ok());

        // Pause should succeed
        let result = engine.pause(session).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_seek() {
        let config = MediaEngineConfig::default();
        let engine = MediaEngineImpl::new(config).unwrap();

        let session = engine
            .create_session(MediaSessionConfig::default())
            .await
            .unwrap();

        // Seek should succeed
        let result = engine.seek(session, Duration::from_secs(10)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_volume_valid() {
        let config = MediaEngineConfig::default();
        let engine = MediaEngineImpl::new(config).unwrap();

        let session = engine
            .create_session(MediaSessionConfig::default())
            .await
            .unwrap();

        // Valid volumes should succeed
        assert!(engine.set_volume(session, 0.0).await.is_ok());
        assert!(engine.set_volume(session, 0.5).await.is_ok());
        assert!(engine.set_volume(session, 1.0).await.is_ok());
    }

    #[tokio::test]
    async fn test_set_volume_invalid() {
        let config = MediaEngineConfig::default();
        let engine = MediaEngineImpl::new(config).unwrap();

        let session = engine
            .create_session(MediaSessionConfig::default())
            .await
            .unwrap();

        // Invalid volumes should fail
        assert!(engine.set_volume(session, -0.1).await.is_err());
        assert!(engine.set_volume(session, 1.1).await.is_err());
    }

    #[tokio::test]
    async fn test_destroy_session() {
        let config = MediaEngineConfig::default();
        let engine = MediaEngineImpl::new(config).unwrap();

        let session = engine
            .create_session(MediaSessionConfig::default())
            .await
            .unwrap();

        // Destroy should succeed
        let result = engine.destroy_session(session).await;
        assert!(result.is_ok());

        // Second destroy should fail (session not found)
        let result = engine.destroy_session(session).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_operation_on_invalid_session() {
        let config = MediaEngineConfig::default();
        let engine = MediaEngineImpl::new(config).unwrap();

        let fake_session = SessionId::new();

        // Operations on non-existent session should fail
        assert!(engine.play(fake_session).await.is_err());
        assert!(engine.pause(fake_session).await.is_err());
        assert!(engine
            .seek(fake_session, Duration::from_secs(0))
            .await
            .is_err());
        assert!(engine.set_volume(fake_session, 0.5).await.is_err());
        assert!(engine.destroy_session(fake_session).await.is_err());
    }

    #[tokio::test]
    async fn test_load_source() {
        let config = MediaEngineConfig::default();
        let engine = MediaEngineImpl::new(config).unwrap();

        let session = engine
            .create_session(MediaSessionConfig::default())
            .await
            .unwrap();

        // Load source should succeed (creates pipeline)
        let source = MediaSource::Url {
            url: "test.mp4".to_string(),
        };
        let result = engine.load_source(session, source).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_sessions() {
        let config = MediaEngineConfig {
            max_sessions: 5,
            ..Default::default()
        };
        let engine = MediaEngineImpl::new(config).unwrap();

        // Create multiple sessions
        let mut sessions = Vec::new();
        for _ in 0..5 {
            let session = engine
                .create_session(MediaSessionConfig::default())
                .await
                .unwrap();
            sessions.push(session);
        }

        // All sessions should be valid
        for session in &sessions {
            assert!(engine.play(*session).await.is_ok());
        }

        // Destroy all sessions
        for session in sessions {
            assert!(engine.destroy_session(session).await.is_ok());
        }
    }
}
