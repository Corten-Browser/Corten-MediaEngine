# Media Engine Architecture - Component Decomposition

## Overview
The Media Engine is decomposed into 12 specialized components following the orchestration system's dependency hierarchy. This enables parallel development while keeping each component under token budget limits.

## Component Dependency Graph

```
Level 0 (Base):
  └─ shared_types (media types, enums, traits)

Level 1 (Core):
  ├─ media_session (session management, state machine)
  ├─ format_parsers (MP4, WebM, Ogg, Matroska demuxers)
  ├─ video_decoders (H.264, H.265, VP8, VP9, AV1)
  ├─ audio_decoders (AAC, MP3, Opus, Vorbis)
  └─ buffer_manager (ring buffers, caches)

Level 2 (Feature):
  ├─ media_pipeline (pipeline orchestration, A/V sync)
  ├─ hardware_accel (VA-API, DXVA, VideoToolbox)
  ├─ webrtc_integration (WebRTC media handling)
  ├─ drm_support (CDM, EME)
  └─ media_capture (screen, camera, microphone)

Level 3 (Integration):
  └─ media_engine (main API, message bus integration)
```

## Component Specifications

### 1. shared_types (Level 0: Base)
**Responsibility**: Common types, enums, traits used across all media components
**Token Estimate**: ~40,000
**Dependencies**: None
**Key Elements**:
- `VideoCodec`, `AudioCodec` enums
- `MediaSource` enum (Url, Buffer, Stream, MSE, WebRTC, Capture)
- `VideoFrame`, `AudioBuffer` structs
- `MediaError`, `MediaEvent` types
- `MediaEngine` trait definition

### 2. media_session (Level 1: Core)
**Responsibility**: Media session lifecycle and state management
**Token Estimate**: ~60,000
**Dependencies**: shared_types
**Key Elements**:
- `MediaSession` struct
- `SessionState` enum (Idle, Loading, Ready, Playing, Paused, Seeking, Ended, Error)
- Session creation, tracking, cleanup
- State transition validation

### 3. format_parsers (Level 1: Core)
**Responsibility**: Container format demuxing and parsing
**Token Estimate**: ~70,000
**Dependencies**: shared_types
**Key Elements**:
- `Demuxer` trait
- `Mp4Demuxer`, `WebmDemuxer`, `OggDemuxer`, `MatroskaDemuxer`
- Container header parsing
- Track extraction
- Packet queuing

### 4. video_decoders (Level 1: Core)
**Responsibility**: Video codec implementations
**Token Estimate**: ~80,000
**Dependencies**: shared_types
**Key Elements**:
- `VideoDecoder` trait
- H.264 decoder (using openh264 or FFmpeg bindings)
- VP8/VP9 decoder (using vpx)
- AV1 decoder (using dav1d/rav1e)
- H.265 decoder (using FFmpeg bindings)
- Software decode fallback

### 5. audio_decoders (Level 1: Core)
**Responsibility**: Audio codec implementations
**Token Estimate**: ~70,000
**Dependencies**: shared_types
**Key Elements**:
- `AudioDecoder` trait
- AAC decoder (using fdk-aac)
- MP3 decoder (using minimp3)
- Opus decoder (using opus)
- Vorbis decoder (using lewton)
- PCM format conversions

### 6. buffer_manager (Level 1: Core)
**Responsibility**: Memory buffer and cache management
**Token Estimate**: ~50,000
**Dependencies**: shared_types
**Key Elements**:
- Ring buffer for streaming data
- Frame cache (video frame pool)
- Audio sample buffer
- Memory usage tracking
- Cache eviction policies

### 7. media_pipeline (Level 2: Feature)
**Responsibility**: Pipeline orchestration and synchronization
**Token Estimate**: ~70,000
**Dependencies**: shared_types, format_parsers, video_decoders, audio_decoders, buffer_manager
**Key Elements**:
- `MediaPipeline` struct
- Source readers (network, file, stream)
- Demuxer coordinator
- Decoder coordinator
- A/V synchronization controller
- Thread pool management
- Pipeline state machine

### 8. hardware_accel (Level 2: Feature)
**Responsibility**: Hardware-accelerated video decoding
**Token Estimate**: ~60,000
**Dependencies**: shared_types, video_decoders
**Key Elements**:
- `HardwareContext` abstraction
- VA-API implementation (Linux)
- DXVA implementation (Windows)
- VideoToolbox implementation (macOS)
- Automatic fallback to software decode
- Hardware capability detection

### 9. webrtc_integration (Level 2: Feature)
**Responsibility**: WebRTC media stream handling
**Token Estimate**: ~60,000
**Dependencies**: shared_types, video_decoders, audio_decoders
**Key Elements**:
- WebRTC encoder/decoder wrappers
- RTP packetization/depacketization
- RTCP handling
- Jitter buffer
- Echo cancellation hooks

### 10. drm_support (Level 2: Feature)
**Responsibility**: DRM and Encrypted Media Extensions
**Token Estimate**: ~60,000
**Dependencies**: shared_types
**Key Elements**:
- `ContentDecryptionModule` (CDM) interface
- EME API implementation
- License acquisition
- Secure decryption pipeline
- Protected frame handling

### 11. media_capture (Level 2: Feature)
**Responsibility**: Device capture APIs
**Token Estimate**: ~60,000
**Dependencies**: shared_types, video_decoders, audio_decoders
**Key Elements**:
- Screen capture (platform-specific)
- Camera capture (V4L2/AVFoundation/DirectShow)
- Microphone capture
- MediaStream API
- Capture constraints handling

### 12. media_engine (Level 3: Integration)
**Responsibility**: Main public API and component coordination
**Token Estimate**: ~70,000
**Dependencies**: All above components
**Key Elements**:
- `MediaEngine` trait implementation
- Session orchestration
- Message bus integration (`MediaEngineMessage`, `MediaEngineEvent`)
- Component lifecycle management
- Error aggregation and reporting
- Performance metrics collection

## Technology Stack

### Core Language
- **Rust** (edition 2021, MSRV 1.70.0)

### Phase 1 Dependencies (GStreamer-based)
- `gstreamer` = "0.22" (GStreamer bindings)
- `gstreamer-app` = "0.22" (AppSink/AppSrc)
- `gstreamer-video` = "0.22" (Video utilities)
- `gstreamer-audio` = "0.22" (Audio utilities)

### Phase 2-3 Dependencies (Rust-native)
**Container Parsers**:
- `mp4` = "0.14" (MP4 parsing)
- `webm-iterable` = "0.6" (WebM parsing)
- `ogg` = "0.9" (Ogg container)
- `matroska` = "0.15" (MKV parsing)

**Video Codecs**:
- `openh264` = "0.3" (H.264 decoder)
- `dav1d` = "0.10" (AV1 decoder)
- `rav1e` = "0.7" (AV1 encoder)
- `vpx` = "0.1" (VP8/VP9 bindings)

**Audio Codecs**:
- `opus` = "0.3" (Opus codec)
- `lewton` = "0.10" (Vorbis decoder)
- `minimp3` = "0.5" (MP3 decoder)
- `fdk-aac` = "0.6" (AAC codec)

**Utilities**:
- `tokio` = { version = "1.35", features = ["full"] } (async runtime)
- `bytes` = "1.5" (byte buffer management)
- `crossbeam-channel` = "0.5" (MPMC channels)
- `parking_lot` = "0.12" (fast mutexes)
- `tracing` = "0.1" (logging)
- `metrics` = "0.21" (performance metrics)

## Build Order

### Phase 1 (Base Layer)
1. **shared_types** - No dependencies

### Phase 2 (Core Layer) - Parallel Development
Launch all in parallel (5 agents):
2. **media_session**
3. **format_parsers**
4. **video_decoders**
5. **audio_decoders**
6. **buffer_manager**

### Phase 3 (Feature Layer) - Parallel Development
Launch all in parallel (5 agents):
7. **media_pipeline**
8. **hardware_accel**
9. **webrtc_integration**
10. **drm_support**
11. **media_capture**

### Phase 4 (Integration Layer)
12. **media_engine** - Depends on all previous components

## Performance Requirements

### Per-Component Metrics
- **format_parsers**: Parse MP4 header in <100ms
- **video_decoders**: 1080p@30fps decode in <30ms per frame
- **audio_decoders**: 48kHz stereo decode in <10ms per frame
- **buffer_manager**: <50MB memory usage for 4K playback
- **media_pipeline**: A/V sync within ±40ms
- **hardware_accel**: 4K@60fps with <5% CPU usage

### System-Wide Targets
- First frame: <500ms
- Seek latency: <100ms (local), <500ms (network)
- Memory usage: <500MB for 4K playback
- Audio/video sync: ±40ms maximum drift

## Testing Strategy

### Component-Level Testing
Each component has:
- Unit tests (80%+ coverage)
- Integration tests (cross-component APIs)
- Contract compliance tests
- Performance benchmarks

### System-Level Testing
- End-to-end playback tests
- Format compatibility tests (MP4, WebM, Ogg, etc.)
- Codec compliance tests (H.264, VP9, AV1, AAC, Opus, etc.)
- Hardware acceleration tests
- WebRTC interoperability tests
- DRM/EME compliance tests
- Web Platform Tests (WPT) media suite

## Implementation Phases

### Milestone 1: GStreamer Foundation (Weeks 1-2)
- Implement `shared_types`
- Implement `media_session`
- Implement `media_engine` with GStreamer backend
- Basic playback working

### Milestone 2: Format Support (Weeks 3-4)
- Implement `format_parsers` (MP4, WebM, Ogg)
- Integrate with GStreamer pipeline
- Progressive download support

### Milestone 3: Native Decoders (Weeks 5-8)
- Implement `video_decoders` (H.264, VP9 priority)
- Implement `audio_decoders` (AAC, Opus priority)
- Implement `buffer_manager`
- Hybrid GStreamer + native decoders

### Milestone 4: Pipeline Optimization (Weeks 9-10)
- Implement `media_pipeline` with native components
- A/V synchronization
- Thread pool optimization

### Milestone 5: Advanced Features (Weeks 11-14)
- Implement `hardware_accel` (VA-API, DXVA, VideoToolbox)
- Implement `webrtc_integration`
- Implement `drm_support`
- Implement `media_capture`

### Milestone 6: Production Readiness (Weeks 15-16)
- Performance optimization
- Memory leak fixes
- WPT test suite passing
- Documentation complete

## API Contracts

Each component exposes clear API contracts in `contracts/` directory:
- `shared_types.yaml` - Common type definitions
- `media_session.yaml` - Session management API
- `format_parsers.yaml` - Demuxer interfaces
- `video_decoders.yaml` - Video decoder interfaces
- `audio_decoders.yaml` - Audio decoder interfaces
- `buffer_manager.yaml` - Buffer management API
- `media_pipeline.yaml` - Pipeline control API
- `hardware_accel.yaml` - Hardware acceleration API
- `webrtc_integration.yaml` - WebRTC interfaces
- `drm_support.yaml` - DRM/EME API
- `media_capture.yaml` - Capture device API
- `media_engine.yaml` - Main public API

## Security Considerations

### Per-Component Security
- **format_parsers**: Input validation, recursion limits, size checks
- **video_decoders**: Bounds checking, memory limits
- **audio_decoders**: Sample rate validation, buffer overflow protection
- **media_pipeline**: Thread safety, race condition prevention
- **drm_support**: Secure memory handling, encrypted buffer protection

### System-Wide Security
- Sandboxed decoder processes
- URL scheme validation
- MIME type validation
- Memory usage limits
- Resource cleanup on errors

## Monitoring & Observability

### Metrics Collection
Each component exports:
- Performance counters (frames decoded, errors, etc.)
- Timing histograms (decode time, seek latency)
- Resource usage (memory, threads, file handles)

### System Aggregation
`media_engine` aggregates metrics to:
- Prometheus format export
- Internal performance dashboard
- Error tracking and alerting

---

**Architecture Version**: 1.0
**Component Count**: 12
**Estimated Total Implementation**: ~760,000 tokens (divided across components)
**Target Completion**: 12-16 weeks with parallel development
**Status**: Ready for component creation
