//! Codec type definitions for video and audio
//!
//! This module provides enumerations for supported video and audio codecs,
//! along with their configuration parameters and profiles.

/// H.264 encoding profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum H264Profile {
    /// Baseline profile - simple encoding, lower quality
    Baseline,
    /// Main profile - standard encoding
    Main,
    /// High profile - advanced encoding, better quality
    High,
    /// High 10 profile - 10-bit color depth
    High10,
    /// High 4:2:2 profile
    High422,
    /// High 4:4:4 profile
    High444,
}

/// H.264 encoding levels (defines resolution and bitrate limits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum H264Level {
    /// Level 3.0 - up to 720p@30fps
    Level3_0,
    /// Level 3.1 - up to 720p@60fps
    Level3_1,
    /// Level 4.0 - up to 1080p@30fps
    Level4_0,
    /// Level 4.1 - up to 1080p@60fps
    Level4_1,
    /// Level 5.0 - up to 4K@30fps
    Level5_0,
    /// Level 5.1 - up to 4K@60fps
    Level5_1,
}

/// H.265 (HEVC) encoding profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum H265Profile {
    /// Main profile - 8-bit color
    Main,
    /// Main 10 profile - 10-bit color
    Main10,
    /// Main Still Picture profile
    MainStillPicture,
}

/// H.265 encoding tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum H265Tier {
    /// Main tier - standard bitrates
    Main,
    /// High tier - higher bitrates
    High,
}

/// H.265 encoding levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum H265Level {
    /// Level 4.0 - up to 1080p@60fps
    Level4_0,
    /// Level 4.1 - up to 1080p@120fps
    Level4_1,
    /// Level 5.0 - up to 4K@30fps
    Level5_0,
    /// Level 5.1 - up to 4K@60fps
    Level5_1,
    /// Level 6.0 - up to 8K@30fps
    Level6_0,
}

/// VP9 encoding profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VP9Profile {
    /// Profile 0 - 8-bit 4:2:0
    Profile0,
    /// Profile 1 - 8-bit 4:2:2 and 4:4:4
    Profile1,
    /// Profile 2 - 10/12-bit 4:2:0
    Profile2,
    /// Profile 3 - 10/12-bit 4:2:2 and 4:4:4
    Profile3,
}

/// AV1 encoding profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AV1Profile {
    /// Main profile - most common
    Main,
    /// High profile - for professional content
    High,
    /// Professional profile - highest quality
    Professional,
}

/// AV1 encoding levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AV1Level {
    /// Level 4.0 - up to 1080p@60fps
    Level4_0,
    /// Level 4.1 - up to 1080p@120fps
    Level4_1,
    /// Level 5.0 - up to 4K@60fps
    Level5_0,
    /// Level 5.1 - up to 4K@120fps
    Level5_1,
}

/// Supported video codec types
///
/// # Examples
///
/// ```
/// use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};
///
/// let codec = VideoCodec::H264 {
///     profile: H264Profile::High,
///     level: H264Level::Level4_1,
///     hardware_accel: true,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum VideoCodec {
    /// H.264/AVC codec
    H264 {
        /// Encoding profile
        profile: H264Profile,
        /// Encoding level
        level: H264Level,
        /// Whether hardware acceleration is enabled
        hardware_accel: bool,
    },
    /// H.265/HEVC codec
    H265 {
        /// Encoding profile
        profile: H265Profile,
        /// Encoding tier
        tier: H265Tier,
        /// Encoding level
        level: H265Level,
    },
    /// VP8 codec (WebM)
    VP8,
    /// VP9 codec (WebM)
    VP9 {
        /// Encoding profile
        profile: VP9Profile,
    },
    /// AV1 codec
    AV1 {
        /// Encoding profile
        profile: AV1Profile,
        /// Encoding level
        level: AV1Level,
    },
    /// Theora codec (Ogg)
    Theora,
}

/// AAC audio encoding profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AACProfile {
    /// Low Complexity - most common
    LC,
    /// High Efficiency - with SBR
    HE,
    /// High Efficiency v2 - with SBR and PS
    HEv2,
    /// Low Delay
    LD,
}

/// MP3 encoding layers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MP3Layer {
    /// Layer 1
    Layer1,
    /// Layer 2
    Layer2,
    /// Layer 3 (most common)
    Layer3,
}

/// Opus application mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpusApplication {
    /// Voice over IP
    VoIP,
    /// General audio
    Audio,
    /// Low delay mode
    LowDelay,
}

/// PCM audio formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PCMFormat {
    /// 32-bit float little-endian
    F32LE,
    /// 16-bit signed integer little-endian
    S16LE,
    /// 24-bit signed integer little-endian
    S24LE,
    /// 32-bit signed integer little-endian
    S32LE,
}

/// Supported audio codec types
///
/// # Examples
///
/// ```
/// use cortenbrowser_shared_types::{AudioCodec, AACProfile};
///
/// let codec = AudioCodec::AAC {
///     profile: AACProfile::LC,
///     sample_rate: 48000,
///     channels: 2,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum AudioCodec {
    /// AAC audio codec
    AAC {
        /// Encoding profile
        profile: AACProfile,
        /// Sample rate in Hz
        sample_rate: u32,
        /// Number of audio channels
        channels: u8,
    },
    /// MP3 audio codec
    MP3 {
        /// Encoding layer
        layer: MP3Layer,
        /// Bitrate in bits per second
        bitrate: u32,
    },
    /// Opus audio codec
    Opus {
        /// Sample rate in Hz
        sample_rate: u32,
        /// Number of audio channels
        channels: u8,
        /// Application mode
        application: OpusApplication,
    },
    /// Vorbis audio codec
    Vorbis,
    /// FLAC lossless audio codec
    FLAC,
    /// PCM uncompressed audio
    PCM {
        /// Sample format
        format: PCMFormat,
        /// Sample rate in Hz
        sample_rate: u32,
        /// Number of audio channels
        channels: u8,
    },
}
