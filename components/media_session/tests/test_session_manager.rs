//! Unit tests for SessionManager

use cortenbrowser_media_session::{MediaMetadata, SessionManager, SessionState};
use cortenbrowser_shared_types::{MediaError, MediaSessionConfig};
use std::time::Duration;

#[test]
fn test_session_manager_new() {
    let manager = SessionManager::new();
    assert!(manager
        .get(cortenbrowser_shared_types::SessionId::new())
        .is_none());
}

#[test]
fn test_session_manager_create() {
    let manager = SessionManager::new();
    let config = MediaSessionConfig::new();

    let result = manager.create(config);
    assert!(result.is_ok());

    let session_id = result.unwrap();
    assert!(manager.get(session_id).is_some());
}

#[test]
fn test_session_manager_get() {
    let manager = SessionManager::new();
    let config = MediaSessionConfig::new();

    let session_id = manager.create(config).unwrap();
    let session = manager.get(session_id);

    assert!(session.is_some());
    assert_eq!(session.unwrap().id, session_id);
}

#[test]
fn test_session_manager_get_nonexistent() {
    let manager = SessionManager::new();
    let fake_id = cortenbrowser_shared_types::SessionId::new();

    let session = manager.get(fake_id);
    assert!(session.is_none());
}

#[test]
fn test_session_manager_destroy() {
    let manager = SessionManager::new();
    let config = MediaSessionConfig::new();

    let session_id = manager.create(config).unwrap();
    assert!(manager.get(session_id).is_some());

    let result = manager.destroy(session_id);
    assert!(result.is_ok());
    assert!(manager.get(session_id).is_none());
}

#[test]
fn test_session_manager_destroy_nonexistent() {
    let manager = SessionManager::new();
    let fake_id = cortenbrowser_shared_types::SessionId::new();

    let result = manager.destroy(fake_id);
    assert!(result.is_ok()); // Destroying nonexistent session is ok
}

#[test]
fn test_session_manager_transition_state_valid() {
    let manager = SessionManager::new();
    let config = MediaSessionConfig::new();
    let session_id = manager.create(config).unwrap();

    let new_state = SessionState::Loading {
        source: cortenbrowser_shared_types::MediaSource::Url {
            url: "test.mp4".to_string(),
        },
        progress: 0.0,
    };

    let result = manager.transition_state(session_id, new_state);
    assert!(result.is_ok());

    let state = manager.get_state(session_id).unwrap();
    assert!(matches!(state, SessionState::Loading { .. }));
}

#[test]
fn test_session_manager_transition_state_invalid() {
    let manager = SessionManager::new();
    let config = MediaSessionConfig::new();
    let session_id = manager.create(config).unwrap();

    // Try to transition from Idle to Playing (invalid)
    let invalid_state = SessionState::Playing {
        position: Duration::ZERO,
        rate: 1.0,
    };

    let result = manager.transition_state(session_id, invalid_state);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        MediaError::InvalidStateTransition { .. }
    ));
}

#[test]
fn test_session_manager_transition_state_nonexistent_session() {
    let manager = SessionManager::new();
    let fake_id = cortenbrowser_shared_types::SessionId::new();

    let new_state = SessionState::Loading {
        source: cortenbrowser_shared_types::MediaSource::Url {
            url: "test.mp4".to_string(),
        },
        progress: 0.0,
    };

    let result = manager.transition_state(fake_id, new_state);
    assert!(result.is_err());
}

#[test]
fn test_session_manager_get_state() {
    let manager = SessionManager::new();
    let config = MediaSessionConfig::new();
    let session_id = manager.create(config).unwrap();

    let state = manager.get_state(session_id);
    assert!(state.is_ok());
    assert_eq!(state.unwrap(), SessionState::Idle);
}

#[test]
fn test_session_manager_get_state_nonexistent() {
    let manager = SessionManager::new();
    let fake_id = cortenbrowser_shared_types::SessionId::new();

    let result = manager.get_state(fake_id);
    assert!(result.is_err());
}

#[test]
fn test_session_manager_multiple_sessions() {
    let manager = SessionManager::new();
    let config = MediaSessionConfig::new();

    let id1 = manager.create(config.clone()).unwrap();
    let id2 = manager.create(config.clone()).unwrap();
    let id3 = manager.create(config).unwrap();

    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);

    assert!(manager.get(id1).is_some());
    assert!(manager.get(id2).is_some());
    assert!(manager.get(id3).is_some());
}

#[test]
fn test_session_manager_full_workflow() {
    let manager = SessionManager::new();
    let config = MediaSessionConfig::new();
    let session_id = manager.create(config).unwrap();

    // Idle -> Loading
    let loading = SessionState::Loading {
        source: cortenbrowser_shared_types::MediaSource::Url {
            url: "test.mp4".to_string(),
        },
        progress: 0.5,
    };
    assert!(manager.transition_state(session_id, loading).is_ok());

    // Loading -> Ready
    let ready = SessionState::Ready {
        duration: Duration::from_secs(60),
        metadata: MediaMetadata::default(),
    };
    assert!(manager.transition_state(session_id, ready).is_ok());

    // Ready -> Playing
    let playing = SessionState::Playing {
        position: Duration::ZERO,
        rate: 1.0,
    };
    assert!(manager.transition_state(session_id, playing).is_ok());

    // Playing -> Paused
    let paused = SessionState::Paused {
        position: Duration::from_secs(10),
    };
    assert!(manager.transition_state(session_id, paused).is_ok());

    // Paused -> Playing
    let playing2 = SessionState::Playing {
        position: Duration::from_secs(10),
        rate: 1.0,
    };
    assert!(manager.transition_state(session_id, playing2).is_ok());

    // Playing -> Ended
    let ended = SessionState::Ended;
    assert!(manager.transition_state(session_id, ended).is_ok());
}
