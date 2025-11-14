# media_pipeline

**Type**: feature
**Tech Stack**: Rust, Tokio, crossbeam
**Version**: 0.1.0

## Responsibility

Pipeline orchestration and A/V synchronization for the Corten Media Engine. This component coordinates:

- Media source readers (file, network, stream)
- Pipeline state machine (Idle → Loading → Ready → Running → Stopped)
- Audio/Video synchronization with configurable threshold (default 40ms)
- Frame and buffer queue management
- Thread pool coordination for decode operations

## Features

- ✅ **MediaPipeline**: Main pipeline orchestrator with async API
- ✅ **AVSyncController**: Audio/video synchronization logic
- ✅ **PipelineConfig**: Configurable buffer size, thread count, and sync threshold
- ✅ **SyncDecision**: Smart decisions for frame display/drop/wait
- ✅ **State Machine**: Safe state transitions with error handling
- ✅ **Queue Management**: Buffered video frame and audio sample queues

## API

### Core Types

- `MediaPipeline` - Main pipeline orchestration struct
- `AVSyncController` - Audio/video synchronization controller
- `PipelineConfig` - Pipeline configuration (buffer size, threads, sync threshold)
- `SyncDecision` - Synchronization decision (Display, Drop, Wait)

## Structure

```
├── src/           # Source code
├── tests/         # Tests (unit, integration)
├── benches/       # Performance benchmarks
├── Cargo.toml     # Rust package configuration
├── CLAUDE.md      # Component-specific instructions for Claude Code
└── README.md      # This file
```

## Usage

### Creating a Pipeline

```rust
use cortenbrowser_media_pipeline::{MediaPipeline, PipelineConfig};
use cortenbrowser_shared_types::MediaSource;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create pipeline with custom configuration
    let config = PipelineConfig {
        buffer_size: 2048,
        thread_count: 4,
        sync_threshold: Duration::from_millis(40),
    };

    let pipeline = MediaPipeline::new(config)?;

    // Load a media source
    let source = MediaSource::Url {
        url: "file:///path/to/video.mp4".to_string(),
    };

    pipeline.load_source(source).await?;

    // Start playback
    pipeline.start().await?;

    // Seek to 10 seconds
    pipeline.seek(Duration::from_secs(10)).await?;

    // Get video frames
    if let Some(frame) = pipeline.get_next_video_frame().await {
        println!("Got frame: {}x{}", frame.width, frame.height);
    }

    // Stop pipeline
    pipeline.stop().await?;

    Ok(())
}
```

### Audio/Video Synchronization

```rust
use cortenbrowser_media_pipeline::{AVSyncController, SyncDecision};
use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
use std::time::Duration;

// Create sync controller
let controller = AVSyncController::new();

// Create a video frame
let frame = VideoFrame {
    width: 1920,
    height: 1080,
    format: PixelFormat::YUV420,
    data: vec![0u8; 1920 * 1080],
    timestamp: Duration::from_millis(1000),
    duration: Some(Duration::from_millis(33)),
    metadata: FrameMetadata::default(),
};

// Check synchronization against audio timestamp
let audio_timestamp = Duration::from_millis(1000);
let decision = controller.sync_frame(&frame, audio_timestamp);

match decision {
    SyncDecision::Display => println!("Display frame now"),
    SyncDecision::Drop => println!("Drop frame (too late)"),
    SyncDecision::Wait { duration } => println!("Wait {:?} before displaying", duration),
}

// Get current playback clock
let clock = controller.get_clock();
println!("Current position: {:?}", clock);
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

Dependencies are defined in `Cargo.toml` and will be added during implementation based on requirements specified in `../../docs/ARCHITECTURE.md`.
