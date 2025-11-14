//! Session state machine implementation

use cortenbrowser_shared_types::{MediaError, MediaSource};
use std::time::Duration;

/// Media metadata associated with a session
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MediaMetadata {
    /// Media title
    pub title: Option<String>,
    /// Media artist
    pub artist: Option<String>,
    /// Media album
    pub album: Option<String>,
    /// Total duration
    pub duration: Duration,
    /// Number of video tracks
    pub video_track_count: usize,
    /// Number of audio tracks
    pub audio_track_count: usize,
}

/// Media session state machine
///
/// Represents the current state of a media session with associated data.
///
/// # State Transitions
///
/// Valid transitions:
/// - Idle → Loading
/// - Loading → Ready | Error
/// - Ready → Playing | Paused
/// - Playing → Paused | Seeking | Ended | Error
/// - Paused → Playing | Seeking | Error
/// - Seeking → Playing | Paused | Error
/// - Any → Error
///
/// # Examples
///
/// ```
/// use cortenbrowser_media_session::SessionState;
/// use cortenbrowser_shared_types::MediaSource;
///
/// let state = SessionState::Idle;
/// let loading = SessionState::Loading {
///     source: MediaSource::Url { url: "test.mp4".to_string() },
///     progress: 0.0,
/// };
///
/// assert!(state.can_transition_to(&loading));
/// ```
#[derive(Debug, Clone, Default)]
pub enum SessionState {
    /// Session is idle, no media loaded
    #[default]
    Idle,

    /// Session is loading media
    Loading {
        /// The media source being loaded
        source: MediaSource,
        /// Loading progress (0.0 to 1.0)
        progress: f32,
    },

    /// Session is ready to play
    Ready {
        /// Media duration
        duration: Duration,
        /// Media metadata
        metadata: MediaMetadata,
    },

    /// Session is actively playing
    Playing {
        /// Current playback position
        position: Duration,
        /// Playback rate (1.0 = normal speed)
        rate: f32,
    },

    /// Session is paused
    Paused {
        /// Current position when paused
        position: Duration,
    },

    /// Session is seeking to a new position
    Seeking {
        /// Target seek position
        target: Duration,
    },

    /// Playback has ended
    Ended,

    /// Session encountered an error
    Error {
        /// The error that occurred
        error: MediaError,
    },
}

impl SessionState {
    /// Checks if a transition to the new state is valid
    ///
    /// # Arguments
    ///
    /// * `new_state` - The target state to transition to
    ///
    /// # Returns
    ///
    /// `true` if the transition is valid, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_media_session::SessionState;
    /// use std::time::Duration;
    ///
    /// let idle = SessionState::Idle;
    /// let playing = SessionState::Playing {
    ///     position: Duration::ZERO,
    ///     rate: 1.0,
    /// };
    ///
    /// // Cannot transition directly from Idle to Playing
    /// assert!(!idle.can_transition_to(&playing));
    /// ```
    pub fn can_transition_to(&self, new_state: &SessionState) -> bool {
        use SessionState::*;

        match (self, new_state) {
            // Any state can transition to Error
            (_, Error { .. }) => true,

            // Idle can only transition to Loading
            (Idle, Loading { .. }) => true,

            // Loading can transition to Ready or Error
            (Loading { .. }, Ready { .. }) => true,

            // Ready can transition to Playing or Paused
            (Ready { .. }, Playing { .. }) => true,
            (Ready { .. }, Paused { .. }) => true,

            // Playing can transition to Paused, Seeking, or Ended
            (Playing { .. }, Paused { .. }) => true,
            (Playing { .. }, Seeking { .. }) => true,
            (Playing { .. }, Ended) => true,

            // Paused can transition to Playing or Seeking
            (Paused { .. }, Playing { .. }) => true,
            (Paused { .. }, Seeking { .. }) => true,

            // Seeking can transition to Playing or Paused
            (Seeking { .. }, Playing { .. }) => true,
            (Seeking { .. }, Paused { .. }) => true,

            // All other transitions are invalid
            _ => false,
        }
    }

    /// Returns the state name as a string
    pub fn state_name(&self) -> &'static str {
        match self {
            SessionState::Idle => "Idle",
            SessionState::Loading { .. } => "Loading",
            SessionState::Ready { .. } => "Ready",
            SessionState::Playing { .. } => "Playing",
            SessionState::Paused { .. } => "Paused",
            SessionState::Seeking { .. } => "Seeking",
            SessionState::Ended => "Ended",
            SessionState::Error { .. } => "Error",
        }
    }
}

impl PartialEq for SessionState {
    fn eq(&self, other: &Self) -> bool {
        use SessionState::*;
        match (self, other) {
            (Idle, Idle) => true,
            (Loading { progress: p1, .. }, Loading { progress: p2, .. }) => p1 == p2,
            (
                Ready {
                    duration: d1,
                    metadata: m1,
                },
                Ready {
                    duration: d2,
                    metadata: m2,
                },
            ) => d1 == d2 && m1 == m2,
            (
                Playing {
                    position: pos1,
                    rate: r1,
                },
                Playing {
                    position: pos2,
                    rate: r2,
                },
            ) => pos1 == pos2 && r1 == r2,
            (Paused { position: p1 }, Paused { position: p2 }) => p1 == p2,
            (Seeking { target: t1 }, Seeking { target: t2 }) => t1 == t2,
            (Ended, Ended) => true,
            (Error { error: e1 }, Error { error: e2 }) => e1 == e2,
            _ => false,
        }
    }
}

impl From<SessionState> for cortenbrowser_shared_types::SessionState {
    fn from(state: SessionState) -> Self {
        match state {
            SessionState::Idle => cortenbrowser_shared_types::SessionState::Idle,
            SessionState::Loading { .. } => cortenbrowser_shared_types::SessionState::Loading,
            SessionState::Ready { .. } => cortenbrowser_shared_types::SessionState::Ready,
            SessionState::Playing { .. } => cortenbrowser_shared_types::SessionState::Playing,
            SessionState::Paused { .. } => cortenbrowser_shared_types::SessionState::Paused,
            SessionState::Seeking { .. } => cortenbrowser_shared_types::SessionState::Seeking,
            SessionState::Ended => cortenbrowser_shared_types::SessionState::Ended,
            SessionState::Error { .. } => cortenbrowser_shared_types::SessionState::Error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state_is_idle() {
        let state = SessionState::default();
        assert_eq!(state, SessionState::Idle);
    }

    #[test]
    fn test_state_name() {
        assert_eq!(SessionState::Idle.state_name(), "Idle");
        assert_eq!(
            SessionState::Playing {
                position: Duration::ZERO,
                rate: 1.0
            }
            .state_name(),
            "Playing"
        );
    }
}
