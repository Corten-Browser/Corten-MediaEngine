//! Integration tests for concurrent session management

use cortenbrowser_media_session::{MediaMetadata, SessionManager, SessionState};
use cortenbrowser_shared_types::{MediaSessionConfig, MediaSource};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_concurrent_session_creation() {
    let manager = Arc::new(SessionManager::new());
    let mut handles = vec![];

    // Create 10 sessions concurrently
    for _ in 0..10 {
        let manager_clone = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let config = MediaSessionConfig::new();
            manager_clone.create(config).unwrap()
        });
        handles.push(handle);
    }

    let session_ids: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify all sessions were created
    assert_eq!(session_ids.len(), 10);

    // Verify all IDs are unique
    for i in 0..session_ids.len() {
        for j in i + 1..session_ids.len() {
            assert_ne!(session_ids[i], session_ids[j]);
        }
    }

    // Verify all sessions can be retrieved
    for id in &session_ids {
        assert!(manager.get(*id).is_some());
    }
}

#[test]
fn test_concurrent_state_transitions() {
    let manager = Arc::new(SessionManager::new());
    let config = MediaSessionConfig::new();
    let session_id = manager.create(config).unwrap();

    // Transition to Loading
    let loading = SessionState::Loading {
        source: MediaSource::Url {
            url: "test.mp4".to_string(),
        },
        progress: 0.0,
    };
    manager.transition_state(session_id, loading).unwrap();

    // Transition to Ready
    let ready = SessionState::Ready {
        duration: Duration::from_secs(60),
        metadata: MediaMetadata::default(),
    };
    manager.transition_state(session_id, ready).unwrap();

    let mut handles = vec![];

    // Try concurrent state transitions
    for i in 0..5 {
        let manager_clone = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let state = if i % 2 == 0 {
                SessionState::Playing {
                    position: Duration::from_secs(i as u64),
                    rate: 1.0,
                }
            } else {
                SessionState::Paused {
                    position: Duration::from_secs(i as u64),
                }
            };

            // Some transitions may fail due to invalid state, that's ok
            let _ = manager_clone.transition_state(session_id, state);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Session should still be accessible
    assert!(manager.get(session_id).is_some());
}

#[test]
fn test_concurrent_get_and_destroy() {
    let manager = Arc::new(SessionManager::new());
    let config = MediaSessionConfig::new();

    // Create multiple sessions
    let session_ids: Vec<_> = (0..5)
        .map(|_| manager.create(config.clone()).unwrap())
        .collect();

    let mut handles = vec![];

    // Concurrently get and destroy sessions
    for (i, id) in session_ids.iter().enumerate() {
        let manager_clone = Arc::clone(&manager);
        let session_id = *id;
        let handle = thread::spawn(move || {
            if i % 2 == 0 {
                // Get session
                manager_clone.get(session_id)
            } else {
                // Destroy session
                manager_clone.destroy(session_id).ok();
                None
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Check that some sessions were destroyed
    let remaining: Vec<_> = session_ids
        .iter()
        .filter(|id| manager.get(**id).is_some())
        .collect();

    assert!(remaining.len() < session_ids.len());
}

#[test]
fn test_session_state_consistency_under_concurrent_access() {
    let manager = Arc::new(SessionManager::new());
    let config = MediaSessionConfig::new();
    let session_id = manager.create(config).unwrap();

    let mut handles = vec![];

    // Multiple threads reading state
    for _ in 0..10 {
        let manager_clone = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let state = manager_clone.get_state(session_id);
            assert!(state.is_ok());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
