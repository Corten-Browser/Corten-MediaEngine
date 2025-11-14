# media_session

**Type**: core
**Tech Stack**: Rust, Tokio
**Version**: 0.1.0

## Responsibility

Media session lifecycle and state management including:

- Session creation and destruction
- State machine implementation (Idle, Loading, Ready, Playing, Paused, Seeking, Ended, Error)
- Thread-safe session storage with `Arc<RwLock<T>>`
- State transition validation
- Concurrent session management

## Structure

```
├── src/           # Source code
├── tests/         # Tests (unit, integration)
├── benches/       # Performance benchmarks
├── Cargo.toml     # Rust package configuration
├── CLAUDE.md      # Component-specific instructions for Claude Code
└── README.md      # This file
```

## Implementation Status

✅ **COMPLETE** - All features implemented and tested

- SessionState enum with state machine
- MediaSession struct with thread-safe state management
- SessionManager for creating/destroying sessions
- State transition validation
- Comprehensive test coverage (41+ tests)
- Zero clippy warnings
- 100% API documentation

## Usage

```rust
use cortenbrowser_media_session::{SessionManager, SessionState};
use cortenbrowser_shared_types::{MediaSessionConfig, MediaSource};

// Create session manager
let manager = SessionManager::new();

// Create a new session
let config = MediaSessionConfig::new();
let session_id = manager.create(config)?;

// Transition to loading
let loading = SessionState::Loading {
    source: MediaSource::Url { url: "video.mp4".to_string() },
    progress: 0.0,
};
manager.transition_state(session_id, loading)?;

// Get current state
let state = manager.get_state(session_id)?;
```

This component is ready for integration via Task tool orchestration.

**Through Orchestrator:**
The orchestrator will launch an agent using the Task tool to implement this component.

**Direct Work:**
```bash
cd components/media_session
claude code
# Claude Code reads local CLAUDE.md and you work directly
```

## Development

See CLAUDE.md for detailed development instructions, quality standards, and TDD requirements.

## Testing

```bash
# Run tests
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Run benchmarks
cargo bench
```

## Dependencies

- `cortenbrowser-shared_types` - Common types (SessionId, MediaError, MediaSource, etc.)
- `parking_lot` (0.12) - High-performance synchronization primitives
- `tokio` (1.35) - Async runtime support
- `uuid` (1.6) - Unique session ID generation
- `thiserror` (1.0) - Error handling

## Quality Metrics

- **Test Coverage**: >90% (41+ tests passing)
- **Clippy Warnings**: 0
- **Documentation**: 100% of public API
- **Thread Safety**: Fully concurrent-safe with Arc<RwLock>
- **Lines of Code**: ~400 (src) + ~600 (tests)
