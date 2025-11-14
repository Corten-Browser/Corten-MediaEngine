///! Types for media engine configuration and messages
use cortenbrowser_buffer_manager::BufferConfig;
use cortenbrowser_media_pipeline::PipelineConfig;
use cortenbrowser_shared_types::{
    AudioBuffer, MediaChunk, MediaElementAttributes, MediaError, PlaybackCommand, SessionId,
    SessionState, VideoFrame,
};

/// Configuration for the Media Engine
#[derive(Debug, Clone)]
pub struct MediaEngineConfig {
    /// Enable hardware acceleration if available
    pub hardware_accel_enabled: bool,
    /// Maximum number of concurrent sessions
    pub max_sessions: usize,
    /// Buffer manager configuration
    pub buffer_config: BufferConfig,
    /// Pipeline configuration
    pub pipeline_config: PipelineConfig,
}

impl Default for MediaEngineConfig {
    fn default() -> Self {
        Self {
            hardware_accel_enabled: true,
            max_sessions: 10,
            buffer_config: BufferConfig::default(),
            pipeline_config: PipelineConfig::default(),
        }
    }
}

/// Messages the Media Engine handles
#[derive(Debug, Clone)]
pub enum MediaEngineMessage {
    /// Create a new media element
    CreateMediaElement {
        /// Element ID
        element_id: String,
        /// Element attributes
        attributes: MediaElementAttributes,
    },
    /// Stream data chunk received
    StreamData {
        /// Session ID
        session_id: SessionId,
        /// Media chunk
        chunk: MediaChunk,
    },
    /// Playback command
    PlaybackCommand {
        /// Session ID
        session_id: SessionId,
        /// Command to execute
        command: PlaybackCommand,
    },
}

/// Events the Media Engine emits
#[derive(Debug, Clone)]
pub enum MediaEngineEvent {
    /// Video frame ready for rendering
    VideoFrameReady {
        /// Session ID
        session_id: SessionId,
        /// Video frame
        frame: VideoFrame,
    },
    /// Audio samples ready for output
    AudioSamplesReady {
        /// Session ID
        session_id: SessionId,
        /// Audio buffer
        buffer: AudioBuffer,
    },
    /// Playback state changed
    PlaybackStateChanged {
        /// Session ID
        session_id: SessionId,
        /// New state
        state: SessionState,
    },
    /// Media error occurred
    MediaError {
        /// Session ID
        session_id: SessionId,
        /// Error details
        error: MediaError,
    },
}
