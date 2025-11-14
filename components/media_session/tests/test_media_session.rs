//! Unit tests for MediaSession

use cortenbrowser_media_session::{MediaSession, SessionState};
use cortenbrowser_shared_types::SessionId;
use std::time::Duration;

#[test]
fn test_media_session_new() {
    let id = SessionId::new();
    let session = MediaSession::new(id);

    assert_eq!(session.id, id);
    assert_eq!(session.get_state(), SessionState::Idle);
}

#[test]
fn test_media_session_set_state() {
    let id = SessionId::new();
    let mut session = MediaSession::new(id);

    let new_state = SessionState::Loading {
        source: cortenbrowser_shared_types::MediaSource::Url {
            url: "test.mp4".to_string(),
        },
        progress: 0.5,
    };

    session.set_state(new_state.clone());
    assert_eq!(session.get_state(), new_state);
}

#[test]
fn test_media_session_get_state() {
    let id = SessionId::new();
    let session = MediaSession::new(id);

    let state = session.get_state();
    assert_eq!(state, SessionState::Idle);
}

#[test]
fn test_media_session_updated_at_changes() {
    let id = SessionId::new();
    let mut session = MediaSession::new(id);
    let initial_time = session.updated_at;

    // Small delay to ensure time difference
    std::thread::sleep(Duration::from_millis(10));

    session.set_state(SessionState::Idle);
    assert!(session.updated_at > initial_time);
}

#[test]
fn test_media_session_clone() {
    let id = SessionId::new();
    let session = MediaSession::new(id);
    let cloned = session.clone();

    assert_eq!(session.id, cloned.id);
    assert_eq!(session.created_at, cloned.created_at);
}

#[test]
fn test_media_session_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let id = SessionId::new();
    let session = Arc::new(MediaSession::new(id));
    let session_clone = Arc::clone(&session);

    let handle = thread::spawn(move || {
        let state = session_clone.get_state();
        assert_eq!(state, SessionState::Idle);
    });

    handle.join().unwrap();
}
