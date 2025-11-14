//! Media session implementation

use crate::state::SessionState;
use cortenbrowser_shared_types::SessionId;
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::SystemTime;

/// Represents a media playback session
#[derive(Debug, Clone)]
pub struct MediaSession {
    /// Unique session identifier
    pub id: SessionId,
    /// Current session state
    pub state: Arc<RwLock<SessionState>>,
    /// Session creation time
    pub created_at: SystemTime,
    /// Last update time
    pub updated_at: SystemTime,
}

impl MediaSession {
    /// Creates a new media session
    pub fn new(id: SessionId) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            state: Arc::new(RwLock::new(SessionState::Idle)),
            created_at: now,
            updated_at: now,
        }
    }

    /// Gets the current state
    pub fn get_state(&self) -> SessionState {
        self.state.read().clone()
    }

    /// Updates the session state
    pub fn set_state(&mut self, new_state: SessionState) {
        *self.state.write() = new_state;
        self.updated_at = SystemTime::now();
    }
}
