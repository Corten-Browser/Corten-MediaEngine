#!/bin/bash
# Script to create all Media Engine components
set -e

PROJECT_ROOT="/home/user/Corten-MediaEngine"
cd "$PROJECT_ROOT"

# Component definitions: name|type|tech_stack|responsibility
COMPONENTS=(
  "shared_types|base|Rust (enums, traits, structs)|Common types, enums, and traits used across all media components (VideoCodec, AudioCodec, MediaSource, VideoFrame, AudioBuffer, MediaError)"
  "media_session|core|Rust, Tokio|Media session lifecycle and state management (session creation, state transitions, cleanup)"
  "format_parsers|core|Rust, mp4 crate, webm-iterable, ogg, matroska|Container format demuxing and parsing (MP4, WebM, Ogg, Matroska)"
  "video_decoders|core|Rust, openh264, dav1d, rav1e, vpx|Video codec implementations (H.264, H.265, VP8, VP9, AV1)"
  "audio_decoders|core|Rust, opus, lewton, minimp3, fdk-aac|Audio codec implementations (AAC, MP3, Opus, Vorbis, PCM)"
  "buffer_manager|core|Rust, bytes, crossbeam|Memory buffer and cache management (ring buffers, frame caches, memory tracking)"
  "media_pipeline|feature|Rust, Tokio, crossbeam|Pipeline orchestration and A/V synchronization (source readers, demuxer/decoder coordination)"
  "hardware_accel|feature|Rust, va-rs (Linux), FFmpeg (cross-platform)|Hardware-accelerated video decoding (VA-API, DXVA, VideoToolbox)"
  "webrtc_integration|feature|Rust, webrtc crate|WebRTC media stream handling (RTP, RTCP, jitter buffer, echo cancellation)"
  "drm_support|feature|Rust, widevine bindings|DRM and Encrypted Media Extensions (CDM interface, license acquisition, secure decryption)"
  "media_capture|feature|Rust, platform-specific APIs|Device capture APIs (screen capture, camera, microphone, MediaStream API)"
  "media_engine|integration|Rust, all above components|Main public API and component coordination (MediaEngine trait, message bus integration, session orchestration)"
)

echo "Creating 12 Media Engine components..."

for component_def in "${COMPONENTS[@]}"; do
  IFS='|' read -r name type tech responsibility <<< "$component_def"

  echo ""
  echo "===== Creating component: $name ====="

  # Create directory structure
  mkdir -p "components/$name/src"
  mkdir -p "components/$name/tests/unit"
  mkdir -p "components/$name/tests/integration"
  mkdir -p "components/$name/benches"

  # Create .gitkeep files
  touch "components/$name/tests/unit/.gitkeep"
  touch "components/$name/tests/integration/.gitkeep"
  touch "components/$name/benches/.gitkeep"

  # Create basic Cargo.toml
  cat > "components/$name/Cargo.toml" << EOF
[package]
name = "cortenbrowser-$name"
version = "0.1.0"
edition = "2021"
authors = ["CortenBrowser Team"]
license = "MIT OR Apache-2.0"

[dependencies]
# Dependencies will be added during implementation

[dev-dependencies]
# Test dependencies will be added during implementation

[features]
default = []
EOF

  # Create basic lib.rs
  cat > "components/$name/src/lib.rs" << EOF
//! # $name Component
//!
//! $responsibility

#![warn(missing_docs)]

// Modules will be added during implementation
EOF

  # Create README.md
  cat > "components/$name/README.md" << EOF
# $name

**Type**: $type
**Tech Stack**: $tech
**Version**: 0.1.0

## Responsibility

$responsibility

## Structure

\`\`\`
├── src/           # Source code
├── tests/         # Tests (unit, integration)
├── benches/       # Performance benchmarks
├── Cargo.toml     # Rust package configuration
├── CLAUDE.md      # Component-specific instructions for Claude Code
└── README.md      # This file
\`\`\`

## Usage

This component is ready for implementation via Task tool orchestration.

**Through Orchestrator:**
The orchestrator will launch an agent using the Task tool to implement this component.

**Direct Work:**
\`\`\`bash
cd components/$name
claude code
# Claude Code reads local CLAUDE.md and you work directly
\`\`\`

## Development

See CLAUDE.md for detailed development instructions, quality standards, and TDD requirements.

## Testing

\`\`\`bash
# Run tests
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Run benchmarks
cargo bench
\`\`\`

## Dependencies

Dependencies are defined in \`Cargo.toml\` and will be added during implementation based on requirements specified in \`../../docs/ARCHITECTURE.md\`.
EOF

  echo "✅ Created: components/$name/"
done

echo ""
echo "✅ All 12 components created successfully!"
echo ""
echo "Directory structure:"
ls -la components/
