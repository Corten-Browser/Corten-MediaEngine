///! Integration tests for media_engine component
use cortenbrowser_media_engine::{MediaEngineConfig, MediaEngineImpl};
use cortenbrowser_shared_types::{MediaEngine, MediaSessionConfig, MediaSource};
use std::time::Duration;

/// Test basic engine creation and configuration
#[tokio::test]
async fn test_engine_initialization() {
    let config = MediaEngineConfig {
        hardware_accel_enabled: true,
        max_sessions: 10,
        ..Default::default()
    };

    let engine = MediaEngineImpl::new(config);
    assert!(engine.is_ok(), "Engine creation should succeed");
}

/// Test complete playback workflow
#[tokio::test]
async fn test_playback_workflow() {
    let engine = MediaEngineImpl::new(MediaEngineConfig::default()).unwrap();

    // Create session
    let session = engine
        .create_session(MediaSessionConfig::default())
        .await
        .expect("Session creation should succeed");

    // Load source
    let source = MediaSource::Url {
        url: "test.mp4".to_string(),
    };
    engine
        .load_source(session, source)
        .await
        .expect("Source loading should succeed");

    // Play
    engine.play(session).await.expect("Play should succeed");

    // Pause
    engine.pause(session).await.expect("Pause should succeed");

    // Seek
    engine
        .seek(session, Duration::from_secs(10))
        .await
        .expect("Seek should succeed");

    // Set volume
    engine
        .set_volume(session, 0.75)
        .await
        .expect("Set volume should succeed");

    // Cleanup
    engine
        .destroy_session(session)
        .await
        .expect("Session cleanup should succeed");
}

/// Test concurrent sessions
#[tokio::test]
async fn test_concurrent_sessions() {
    let engine = MediaEngineImpl::new(MediaEngineConfig {
        max_sessions: 3,
        ..Default::default()
    })
    .unwrap();

    // Create multiple sessions concurrently
    let session1 = engine
        .create_session(MediaSessionConfig::default())
        .await
        .unwrap();
    let session2 = engine
        .create_session(MediaSessionConfig::default())
        .await
        .unwrap();
    let session3 = engine
        .create_session(MediaSessionConfig::default())
        .await
        .unwrap();

    // All should be playable
    assert!(engine.play(session1).await.is_ok());
    assert!(engine.play(session2).await.is_ok());
    assert!(engine.play(session3).await.is_ok());

    // Cleanup
    assert!(engine.destroy_session(session1).await.is_ok());
    assert!(engine.destroy_session(session2).await.is_ok());
    assert!(engine.destroy_session(session3).await.is_ok());
}

/// Test session limit enforcement
#[tokio::test]
async fn test_session_limit_enforcement() {
    let engine = MediaEngineImpl::new(MediaEngineConfig {
        max_sessions: 2,
        ..Default::default()
    })
    .unwrap();

    // Create sessions up to limit
    let session1 = engine.create_session(MediaSessionConfig::default()).await;
    assert!(session1.is_ok(), "First session should succeed");

    let session2 = engine.create_session(MediaSessionConfig::default()).await;
    assert!(session2.is_ok(), "Second session should succeed");

    // Third should fail
    let session3 = engine.create_session(MediaSessionConfig::default()).await;
    assert!(session3.is_err(), "Third session should fail due to limit");

    // Cleanup one session
    engine.destroy_session(session1.unwrap()).await.unwrap();

    // Now another should succeed
    let session4 = engine.create_session(MediaSessionConfig::default()).await;
    assert!(session4.is_ok(), "Session should succeed after cleanup");
}

/// Test error handling for invalid operations
#[tokio::test]
async fn test_invalid_operations() {
    let engine = MediaEngineImpl::new(MediaEngineConfig::default()).unwrap();

    let session = engine
        .create_session(MediaSessionConfig::default())
        .await
        .unwrap();

    // Invalid volume values
    assert!(
        engine.set_volume(session, -0.5).await.is_err(),
        "Negative volume should fail"
    );
    assert!(
        engine.set_volume(session, 1.5).await.is_err(),
        "Volume > 1.0 should fail"
    );
}

/// Test session state transitions
#[tokio::test]
async fn test_state_transitions() {
    let engine = MediaEngineImpl::new(MediaEngineConfig::default()).unwrap();

    let session = engine
        .create_session(MediaSessionConfig::default())
        .await
        .unwrap();

    // Play -> Pause -> Play should work
    assert!(engine.play(session).await.is_ok());
    assert!(engine.pause(session).await.is_ok());
    assert!(engine.play(session).await.is_ok());

    // Seek while playing
    assert!(engine.seek(session, Duration::from_secs(5)).await.is_ok());

    engine.destroy_session(session).await.unwrap();
}
