//! Pixel and audio format definitions
//!
//! This module provides enumerations for pixel formats (color spaces) and
//! audio sample formats used in media processing.

/// Pixel format (color space) for video frames
///
/// # Examples
///
/// ```
/// use cortenbrowser_shared_types::PixelFormat;
///
/// let format = PixelFormat::YUV420; // Common format for video
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// YUV 4:2:0 planar format (most common for video)
    ///
    /// Y plane followed by U and V planes at half resolution
    YUV420,

    /// YUV 4:2:2 planar format
    ///
    /// Y plane followed by U and V planes at half horizontal resolution
    YUV422,

    /// YUV 4:4:4 planar format
    ///
    /// Y, U, and V planes all at full resolution
    YUV444,

    /// RGB 24-bit packed format
    ///
    /// 8 bits per channel, packed as RGB (3 bytes per pixel)
    RGB24,

    /// RGBA 32-bit packed format
    ///
    /// 8 bits per channel including alpha, packed as RGBA (4 bytes per pixel)
    RGBA32,

    /// NV12 semi-planar format
    ///
    /// Y plane followed by interleaved UV plane
    /// Common for hardware decoders
    NV12,
}

impl PixelFormat {
    /// Returns the number of bytes per pixel for packed formats
    ///
    /// Returns None for planar formats where this concept doesn't apply
    pub fn bytes_per_pixel(&self) -> Option<usize> {
        match self {
            PixelFormat::RGB24 => Some(3),
            PixelFormat::RGBA32 => Some(4),
            _ => None, // Planar formats don't have a fixed bpp
        }
    }

    /// Returns whether this is a planar format
    pub fn is_planar(&self) -> bool {
        matches!(
            self,
            PixelFormat::YUV420 | PixelFormat::YUV422 | PixelFormat::YUV444 | PixelFormat::NV12
        )
    }

    /// Returns whether this is an RGB format
    pub fn is_rgb(&self) -> bool {
        matches!(self, PixelFormat::RGB24 | PixelFormat::RGBA32)
    }
}

/// Audio sample format
///
/// # Examples
///
/// ```
/// use cortenbrowser_shared_types::AudioFormat;
///
/// let format = AudioFormat::F32LE; // 32-bit float samples
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioFormat {
    /// 32-bit floating-point samples, little-endian
    ///
    /// Range: -1.0 to 1.0
    /// Most flexible for processing
    F32LE,

    /// 16-bit signed integer samples, little-endian
    ///
    /// Range: -32768 to 32767
    /// Common for audio files and streaming
    S16LE,

    /// 24-bit signed integer samples, little-endian
    ///
    /// Range: -8388608 to 8388607
    /// High quality audio
    S24LE,

    /// 32-bit signed integer samples, little-endian
    ///
    /// Range: -2147483648 to 2147483647
    /// Professional audio
    S32LE,
}

impl AudioFormat {
    /// Returns the number of bytes per sample
    pub fn bytes_per_sample(&self) -> usize {
        match self {
            AudioFormat::F32LE => 4,
            AudioFormat::S16LE => 2,
            AudioFormat::S24LE => 3,
            AudioFormat::S32LE => 4,
        }
    }

    /// Returns whether this is a floating-point format
    pub fn is_float(&self) -> bool {
        matches!(self, AudioFormat::F32LE)
    }

    /// Returns whether this is an integer format
    pub fn is_integer(&self) -> bool {
        !self.is_float()
    }
}
