//! Hardware capabilities reporting

use cortenbrowser_shared_types::VideoCodec;

/// Hardware acceleration capabilities
///
/// Describes the capabilities of the available hardware acceleration,
/// including supported codecs, maximum resolution, and frame rate.
///
/// # Examples
///
/// ```
/// use cortenbrowser_hardware_accel::HardwareCapabilities;
/// use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};
///
/// let caps = HardwareCapabilities {
///     supported_codecs: vec![
///         VideoCodec::H264 {
///             profile: H264Profile::High,
///             level: H264Level::Level4_1,
///             hardware_accel: true,
///         },
///     ],
///     max_resolution: (3840, 2160), // 4K
///     max_framerate: 60.0,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct HardwareCapabilities {
    /// List of video codecs supported by hardware
    pub supported_codecs: Vec<VideoCodec>,

    /// Maximum resolution supported (width, height)
    pub max_resolution: (u32, u32),

    /// Maximum frame rate supported (frames per second)
    pub max_framerate: f32,
}

impl Default for HardwareCapabilities {
    fn default() -> Self {
        Self {
            supported_codecs: Vec::new(),
            max_resolution: (0, 0),
            max_framerate: 0.0,
        }
    }
}
