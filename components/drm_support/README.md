# DRM Support Component

**Version**: 0.1.0
**Type**: Generic Component
**Tech Stack**: Rust, tokio (async runtime), EME/CDM interfaces
**Test Coverage**: 100% (36 tests passing)
**Status**: Implemented (stub for development/testing)

## Overview

The DRM Support component provides Digital Rights Management (DRM) capabilities for the Corten Media Engine, implementing the W3C Encrypted Media Extensions (EME) specification. It handles DRM session management, license acquisition, and provides a decryption interface for protected media content.

### Key Features

- ✅ **EME Interface**: Request access to key systems (Widevine, PlayReady, FairPlay, ClearKey)
- ✅ **Content Decryption Module (CDM)**: Manage DRM sessions and decryption lifecycle
- ✅ **Session Management**: Create, track, and manage DRM session states
- ✅ **License Acquisition**: Generate license requests and process server responses
- ⚠️  **Stub Decryption**: Placeholder for platform-specific secure decryption (production requires platform CDM)

## Public API

### Quick Example

```rust
use cortenbrowser_drm_support::{
    ContentDecryptionModule, EMEInterface, MediaKeySystemConfiguration
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Request key system access
    let eme = EMEInterface::new();
    let access = eme.request_media_key_system_access(
        "com.widevine.alpha".to_string(),
        vec![MediaKeySystemConfiguration::default()]
    ).await?;

    // Create CDM and session
    let cdm = ContentDecryptionModule::new(access.key_system().to_string())?;
    let session_id = cdm.create_session().await?;

    // Generate license request
    let license_request = cdm.generate_request(&session_id, b"init_data").await?;

    // Update with license response
    cdm.update(&session_id, b"license_response").await?;

    // Decrypt content
    let decrypted = cdm.decrypt(b"encrypted_data", b"key_id")?;

    Ok(())
}
```

### Supported Key Systems

- `com.widevine.alpha` - Widevine DRM
- `com.microsoft.playready` - PlayReady DRM
- `com.apple.fps` - FairPlay Streaming
- `org.w3.clearkey` - ClearKey (for testing)

## Structure

```
drm_support/
├── src/
│   ├── lib.rs          # Public API exports and documentation
│   ├── types.rs        # Core DRM types (DrmSessionId, DrmError, SessionState)
│   ├── cdm.rs          # ContentDecryptionModule implementation
│   └── eme.rs          # EMEInterface and key system access
├── tests/
│   ├── unit/           # Unit tests (23 tests)
│   ├── integration/    # Integration tests (4 tests)
│   ├── unit_tests.rs   # Unit test harness
│   └── integration_tests.rs  # Integration test harness
├── Cargo.toml         # Dependencies and configuration
├── CLAUDE.md          # Component-specific development instructions
└── README.md          # This file
```

## Development

### Running Tests

```bash
# Run all tests (36 tests)
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run clippy (linter)
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Test Results

- **Unit Tests**: 27 passing
- **Integration Tests**: 4 passing
- **Module Tests**: 9 passing (embedded in src/)
- **Total**: 36 tests, 100% pass rate
- **Lint**: 0 warnings

## Security Considerations

**⚠️ IMPORTANT**: This is a **stub implementation** for development and testing.

### Production Requirements

Production deployment **MUST**:
- Use platform-specific CDM (Widevine, PlayReady, FairPlay)
- Implement secure decryption in hardware TEE/secure enclave
- Never expose clear decryption keys in memory
- Validate HDCP and output protection requirements
- Implement secure video path for protected content

### Current Limitations

- ✅ Implements EME API correctly
- ✅ Manages DRM session lifecycle
- ✅ Generates license request format
- ❌ Does NOT perform actual decryption
- ❌ Does NOT integrate with platform CDM
- ❌ Does NOT enforce output protection

## Dependencies

See `Cargo.toml` for complete list. Key dependencies:
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization
- `uuid` - Session ID generation
- `thiserror` - Error handling
- `cortenbrowser-shared_types` - Shared types
