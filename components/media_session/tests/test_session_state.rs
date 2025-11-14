//! Unit tests for SessionState state machine

use cortenbrowser_media_session::{MediaMetadata, SessionState};
use cortenbrowser_shared_types::{MediaError, MediaSource};
use std::time::Duration;

#[test]
fn test_session_state_starts_idle() {
    let state = SessionState::Idle;
    assert_eq!(state, SessionState::Idle);
}

#[test]
fn test_session_state_transition_idle_to_loading() {
    let state = SessionState::Idle;
    let new_state = SessionState::Loading {
        source: MediaSource::Url {
            url: "test.mp4".to_string(),
        },
        progress: 0.0,
    };

    assert!(state.can_transition_to(&new_state));
}

#[test]
fn test_session_state_transition_loading_to_ready() {
    let state = SessionState::Loading {
        source: MediaSource::Url {
            url: "test.mp4".to_string(),
        },
        progress: 1.0,
    };

    let new_state = SessionState::Ready {
        duration: Duration::from_secs(60),
        metadata: MediaMetadata::default(),
    };

    assert!(state.can_transition_to(&new_state));
}

#[test]
fn test_session_state_transition_ready_to_playing() {
    let state = SessionState::Ready {
        duration: Duration::from_secs(60),
        metadata: MediaMetadata::default(),
    };

    let new_state = SessionState::Playing {
        position: Duration::ZERO,
        rate: 1.0,
    };

    assert!(state.can_transition_to(&new_state));
}

#[test]
fn test_session_state_transition_playing_to_paused() {
    let state = SessionState::Playing {
        position: Duration::from_secs(10),
        rate: 1.0,
    };

    let new_state = SessionState::Paused {
        position: Duration::from_secs(10),
    };

    assert!(state.can_transition_to(&new_state));
}

#[test]
fn test_session_state_transition_paused_to_playing() {
    let state = SessionState::Paused {
        position: Duration::from_secs(10),
    };

    let new_state = SessionState::Playing {
        position: Duration::from_secs(10),
        rate: 1.0,
    };

    assert!(state.can_transition_to(&new_state));
}

#[test]
fn test_session_state_transition_to_seeking() {
    let state = SessionState::Playing {
        position: Duration::from_secs(10),
        rate: 1.0,
    };

    let new_state = SessionState::Seeking {
        target: Duration::from_secs(30),
    };

    assert!(state.can_transition_to(&new_state));
}

#[test]
fn test_session_state_transition_seeking_to_playing() {
    let state = SessionState::Seeking {
        target: Duration::from_secs(30),
    };

    let new_state = SessionState::Playing {
        position: Duration::from_secs(30),
        rate: 1.0,
    };

    assert!(state.can_transition_to(&new_state));
}

#[test]
fn test_session_state_transition_to_ended() {
    let state = SessionState::Playing {
        position: Duration::from_secs(60),
        rate: 1.0,
    };

    let new_state = SessionState::Ended;

    assert!(state.can_transition_to(&new_state));
}

#[test]
fn test_session_state_transition_to_error() {
    let state = SessionState::Playing {
        position: Duration::from_secs(10),
        rate: 1.0,
    };

    let new_state = SessionState::Error {
        error: MediaError::CodecError {
            details: "Test error".to_string(),
        },
    };

    assert!(state.can_transition_to(&new_state));
}

#[test]
fn test_invalid_state_transition_idle_to_playing() {
    let state = SessionState::Idle;
    let new_state = SessionState::Playing {
        position: Duration::ZERO,
        rate: 1.0,
    };

    assert!(!state.can_transition_to(&new_state));
}

#[test]
fn test_invalid_state_transition_ended_to_playing() {
    let state = SessionState::Ended;
    let new_state = SessionState::Playing {
        position: Duration::ZERO,
        rate: 1.0,
    };

    assert!(!state.can_transition_to(&new_state));
}

#[test]
fn test_session_state_clone() {
    let state = SessionState::Playing {
        position: Duration::from_secs(10),
        rate: 1.5,
    };

    let cloned = state.clone();
    assert_eq!(state, cloned);
}

#[test]
fn test_session_state_debug_format() {
    let state = SessionState::Idle;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Idle"));
}
