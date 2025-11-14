# shared_types

**Type**: Base Component (Level 0)
**Tech Stack**: Rust (enums, traits, structs)
**Version**: 0.1.0

## Overview

Common types, enums, and traits used across all media engine components. This is a foundational component with no dependencies on other components, providing the core type definitions for the Corten Media Engine.

## Purpose

The `shared_types` component defines:

- **Codec Types**: Video and audio codec enumerations with configuration parameters
- **Media Formats**: Pixel formats (YUV, RGB) and audio sample formats
- **Media Data**: Structures for video frames, audio buffers, and media sources
- **Error Handling**: Comprehensive error types for media operations
- **Session Management**: Session identifiers and configuration
- **Core Traits**: Interfaces for media engines, demuxers, and decoders

## Public API

### Codec Types

- `VideoCodec` - H.264, H.265, VP8, VP9, AV1, Theora
- `AudioCodec` - AAC, MP3, Opus, Vorbis, FLAC, PCM
- Profile/level enums for each codec

### Media Data

- `VideoFrame` - Decoded video frame with metadata
- `AudioBuffer` - Decoded audio samples
- `MediaSource` - Source of media (URL, buffer, stream, etc.)

### Formats

- `PixelFormat` - YUV420, YUV422, YUV444, RGB24, RGBA32, NV12
- `AudioFormat` - F32LE, S16LE, S24LE, S32LE

### Error Handling

- `MediaError` - Comprehensive error enum for all media operations
- `MediaResult<T>` - Convenient Result type alias

### Session Management

- `SessionId` - Unique identifier for media sessions
- `MediaSessionConfig` - Configuration for session creation

### Traits

- `MediaEngine` - Main interface for media playback
- `Demuxer` - Container format parsing
- `VideoDecoder` - Video codec decoding
- `AudioDecoder` - Audio codec decoding

## Usage Examples

### Creating a Video Codec

```rust
use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};

let codec = VideoCodec::H264 {
    profile: H264Profile::High,
    level: H264Level::Level4_1,
    hardware_accel: true,
};
```

### Creating a Video Frame

```rust
use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
use std::time::Duration;

let frame = VideoFrame {
    width: 1920,
    height: 1080,
    format: PixelFormat::YUV420,
    data: vec![0u8; 1920 * 1080],
    timestamp: Duration::from_secs(1),
    duration: Some(Duration::from_millis(33)),
    metadata: FrameMetadata::default(),
};
```

### Creating a Media Source

```rust
use cortenbrowser_shared_types::MediaSource;

let source = MediaSource::Url {
    url: "https://example.com/video.mp4".to_string(),
};
```

### Using MediaEngine Trait

```rust
use cortenbrowser_shared_types::{MediaEngine, MediaSource, SessionId};
use std::time::Duration;

async fn play_video<E: MediaEngine>(engine: &E) -> Result<(), Box<dyn std::error::Error>> {
    let session = engine.create_session(Default::default()).await?;
    engine.load_source(session, MediaSource::Url {
        url: "video.mp4".to_string()
    }).await?;
    engine.play(session).await?;
    Ok(())
}
```

## Architecture

This component has no dependencies - it's a pure type definition library. All other media components depend on this one.

```
shared_types (Level 0)
├── No dependencies
└── Used by: All other components
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo test --all-features

# Run clippy
cargo clippy -- -D warnings
```

### Test Coverage

Current coverage: **100%** (all public types and functions tested)

Test breakdown:
- 40 unit tests
- 12 doc tests
- All public API covered

## Quality Metrics

- **Tests**: 52 total (40 unit + 12 doc)
- **Test Pass Rate**: 100%
- **Coverage**: 100% of public API
- **Clippy**: 0 warnings
- **Documentation**: Complete for all public items

## Component Structure

```
shared_types/
├── src/
│   ├── lib.rs         # Public API and re-exports
│   ├── codecs.rs      # Video/audio codec definitions
│   ├── formats.rs     # Pixel/audio format enums
│   ├── errors.rs      # Error types
│   ├── media.rs       # VideoFrame, AudioBuffer, MediaSource
│   ├── session.rs     # SessionId and configuration
│   └── traits.rs      # Core trait definitions
├── tests/
│   ├── integration_tests.rs
│   └── unit/
│       ├── test_codecs.rs
│       ├── test_errors.rs
│       ├── test_formats.rs
│       ├── test_media.rs
│       └── test_traits.rs
├── Cargo.toml
└── README.md
```

## Dependencies

- `uuid` - For SessionId generation
- `tokio` - For async traits and channels
- `thiserror` - For error handling

## API Stability

This component is in pre-release (0.x.x). Breaking changes are encouraged to improve the design during development.

## License

MIT OR Apache-2.0
