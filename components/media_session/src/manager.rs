//! Session manager implementation

use crate::session::MediaSession;
use crate::state::SessionState;
use cortenbrowser_shared_types::{MediaError, MediaSessionConfig, SessionId};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// Manages media sessions
#[derive(Debug, Default)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<SessionId, Arc<MediaSession>>>>,
}

impl SessionManager {
    /// Creates a new session manager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new media session
    pub fn create(&self, _config: MediaSessionConfig) -> Result<SessionId, MediaError> {
        let id = SessionId::new();
        let session = Arc::new(MediaSession::new(id));
        self.sessions.write().insert(id, session);
        Ok(id)
    }

    /// Gets an existing session
    pub fn get(&self, id: SessionId) -> Option<Arc<MediaSession>> {
        self.sessions.read().get(&id).cloned()
    }

    /// Destroys and cleans up a session
    pub fn destroy(&self, id: SessionId) -> Result<(), MediaError> {
        self.sessions.write().remove(&id);
        Ok(())
    }

    /// Transitions session state with validation
    pub fn transition_state(
        &self,
        id: SessionId,
        new_state: SessionState,
    ) -> Result<(), MediaError> {
        let sessions = self.sessions.read();
        let session = sessions.get(&id).ok_or_else(|| MediaError::CodecError {
            details: "Session not found".to_string(),
        })?;

        let current_state = session.state.read().clone();

        if !current_state.can_transition_to(&new_state) {
            return Err(MediaError::InvalidStateTransition {
                from: current_state.into(),
                to: new_state.into(),
            });
        }

        *session.state.write() = new_state;
        Ok(())
    }

    /// Gets current session state
    pub fn get_state(&self, id: SessionId) -> Result<SessionState, MediaError> {
        let sessions = self.sessions.read();
        let session = sessions.get(&id).ok_or_else(|| MediaError::CodecError {
            details: "Session not found".to_string(),
        })?;

        let state = session.state.read().clone();
        Ok(state)
    }
}
