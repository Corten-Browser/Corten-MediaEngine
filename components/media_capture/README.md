# media_capture

**Type**: feature
**Tech Stack**: Rust, platform-specific APIs (V4L2, AVFoundation, DirectShow)
**Version**: 0.1.0

## Responsibility

Device capture APIs providing screen capture, camera capture, and microphone capture capabilities with platform-specific implementations.

## Features

- **DeviceEnumerator**: List available video and audio input devices
- **ScreenCapture**: Capture video frames from the screen
- **CameraCapture**: Capture video frames from cameras/webcams
- **MicrophoneCapture**: Capture audio samples from microphones
- **CaptureConstraints**: Configure video capture (resolution, frame rate)
- **AudioConstraints**: Configure audio capture (sample rate, channels)

## Structure

```
├── src/
│   ├── lib.rs                     # Public API exports
│   ├── types.rs                   # Type definitions
│   ├── device_enumerator.rs       # Device enumeration
│   ├── screen_capture.rs          # Screen capture
│   ├── camera_capture.rs          # Camera capture
│   └── microphone_capture.rs      # Microphone capture
├── tests/
│   ├── lib.rs                     # Test entry point
│   └── unit/                      # Unit tests
│       ├── test_types.rs
│       ├── test_device_enumerator.rs
│       ├── test_screen_capture.rs
│       ├── test_camera_capture.rs
│       └── test_microphone_capture.rs
├── benches/                       # Performance benchmarks
├── Cargo.toml                     # Rust package configuration
├── CLAUDE.md                      # Component-specific instructions for Claude Code
└── README.md                      # This file
```

## Usage

### Device Enumeration

```rust
use cortenbrowser_media_capture::DeviceEnumerator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let enumerator = DeviceEnumerator::new();

    let video_devices = enumerator.enumerate_video_devices().await?;
    for device in video_devices {
        println!("Video: {} ({})", device.label, device.device_id);
    }

    let audio_devices = enumerator.enumerate_audio_devices().await?;
    for device in audio_devices {
        println!("Audio: {} ({})", device.label, device.device_id);
    }

    Ok(())
}
```

### Screen Capture

```rust
use cortenbrowser_media_capture::{ScreenCapture, CaptureConstraints};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let constraints = CaptureConstraints {
        width: Some(1920),
        height: Some(1080),
        frame_rate: Some(30.0),
    };

    let capture = ScreenCapture::new(constraints)?;
    let mut receiver = capture.start().await?;

    while let Some(frame) = receiver.recv().await {
        println!("Frame: {}x{}", frame.width, frame.height);
    }

    capture.stop()?;
    Ok(())
}
```

### Camera Capture

```rust
use cortenbrowser_media_capture::{CameraCapture, CaptureConstraints};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device_id = "camera-001".to_string();
    let constraints = CaptureConstraints {
        width: Some(1280),
        height: Some(720),
        frame_rate: Some(30.0),
    };

    let capture = CameraCapture::new(device_id, constraints)?;
    let mut receiver = capture.start().await?;

    while let Some(frame) = receiver.recv().await {
        println!("Frame: {}x{}", frame.width, frame.height);
    }

    capture.stop()?;
    Ok(())
}
```

### Microphone Capture

```rust
use cortenbrowser_media_capture::{MicrophoneCapture, AudioConstraints};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device_id = "mic-001".to_string();
    let constraints = AudioConstraints {
        sample_rate: Some(48000),
        channels: Some(2),
    };

    let capture = MicrophoneCapture::new(device_id, constraints)?;
    let mut receiver = capture.start().await?;

    while let Some(buffer) = receiver.recv().await {
        println!("Samples: {}", buffer.samples.len());
    }

    capture.stop()?;
    Ok(())
}
```

## API

### Types

- `CaptureConstraints` - Video capture constraints (width, height, frame_rate)
- `AudioConstraints` - Audio capture constraints (sample_rate, channels)
- `DeviceInfo` - Device information (device_id, label, kind)
- `DeviceKind` - Device type (VideoInput, AudioInput, AudioOutput)
- `CaptureError` - Error types (DeviceNotFound, PermissionDenied, CaptureFailure)

### Interfaces

- `DeviceEnumerator::new()` - Create device enumerator
- `DeviceEnumerator::enumerate_video_devices()` - List video devices
- `DeviceEnumerator::enumerate_audio_devices()` - List audio devices
- `ScreenCapture::new(constraints)` - Create screen capture
- `ScreenCapture::start()` - Start capturing
- `ScreenCapture::stop()` - Stop capturing
- `CameraCapture::new(device_id, constraints)` - Create camera capture
- `CameraCapture::start()` - Start capturing
- `CameraCapture::stop()` - Stop capturing
- `MicrophoneCapture::new(device_id, constraints)` - Create microphone capture
- `MicrophoneCapture::start()` - Start capturing
- `MicrophoneCapture::stop()` - Stop capturing

## Implementation Status

**Current**: Platform-independent stubs for testing and integration
**Next**: Platform-specific implementations (V4L2, AVFoundation, DirectShow)

## Development

See CLAUDE.md for detailed development instructions, quality standards, and TDD requirements.

## Testing

```bash
# Run all tests
cargo test

# Run tests quietly
cargo test --quiet

# Run linter
cargo clippy -- -D warnings

# Run with coverage
cargo tarpaulin --out Html

# Run benchmarks
cargo bench
```

### Test Coverage

- 23 unit tests covering all components
- 20 documentation tests
- 100% test pass rate
- Zero clippy warnings

## Dependencies

- `tokio` - Async runtime for capture operations
- `cortenbrowser-shared_types` - Shared types (VideoFrame, AudioBuffer)
