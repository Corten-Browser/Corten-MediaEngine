//! Hardware context for platform detection and decoder creation

use crate::capabilities::HardwareCapabilities;
use crate::error::{HardwareError, HardwareResult};
use cortenbrowser_shared_types::{H264Level, H264Profile, VP9Profile, VideoCodec, VideoDecoder};

#[cfg(target_os = "linux")]
use crate::vaapi::VAAPIDecoder;

#[cfg(target_os = "windows")]
use crate::dxva::DXVADecoder;

#[cfg(target_os = "macos")]
use crate::videotoolbox::VideoToolboxDecoder;

/// Hardware acceleration context
///
/// Provides platform detection and hardware decoder creation.
/// Automatically detects the available hardware acceleration API
/// based on the operating system:
/// - Linux: VA-API
/// - Windows: DXVA (stub)
/// - macOS: VideoToolbox (stub)
///
/// # Examples
///
/// ```no_run
/// use cortenbrowser_hardware_accel::HardwareContext;
/// use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let ctx = HardwareContext::new()?;
///
/// let h264 = VideoCodec::H264 {
///     profile: H264Profile::High,
///     level: H264Level::Level4_1,
///     hardware_accel: true,
/// };
///
/// if ctx.is_codec_supported(&h264) {
///     let decoder = ctx.create_decoder(&h264)?;
///     // Use decoder...
/// }
/// # Ok(())
/// # }
/// ```
pub struct HardwareContext {
    capabilities: HardwareCapabilities,
}

impl HardwareContext {
    /// Create a new hardware context
    ///
    /// Initializes hardware acceleration based on the platform.
    /// Returns `Err(HardwareError::NotAvailable)` if hardware acceleration
    /// is not available on this system.
    ///
    /// # Errors
    ///
    /// Returns `HardwareError::NotAvailable` if:
    /// - The platform is not supported
    /// - Hardware drivers are not installed
    /// - Hardware initialization fails
    pub fn new() -> HardwareResult<Self> {
        #[cfg(target_os = "linux")]
        {
            Self::init_linux()
        }

        #[cfg(target_os = "windows")]
        {
            Self::init_windows()
        }

        #[cfg(target_os = "macos")]
        {
            Self::init_macos()
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Err(HardwareError::NotAvailable)
        }
    }

    /// Initialize hardware context for Linux (VA-API)
    #[cfg(target_os = "linux")]
    fn init_linux() -> HardwareResult<Self> {
        // Attempt to detect VA-API capabilities
        // For now, we return a conservative set of capabilities
        // In a full implementation, this would query VA-API directly

        let mut capabilities = HardwareCapabilities::default();

        // Common VA-API supported codecs
        capabilities.supported_codecs = vec![
            VideoCodec::H264 {
                profile: H264Profile::High,
                level: H264Level::Level5_1,
                hardware_accel: true,
            },
            VideoCodec::VP9 {
                profile: VP9Profile::Profile0,
            },
        ];

        capabilities.max_resolution = (4096, 4096); // Typical VA-API max
        capabilities.max_framerate = 60.0;

        Ok(Self { capabilities })
    }

    /// Initialize hardware context for Windows (DXVA stub)
    #[cfg(target_os = "windows")]
    fn init_windows() -> HardwareResult<Self> {
        // DXVA implementation is a stub for now
        // TODO: Implement DXVA support
        Err(HardwareError::NotAvailable)
    }

    /// Initialize hardware context for macOS (VideoToolbox stub)
    #[cfg(target_os = "macos")]
    fn init_macos() -> HardwareResult<Self> {
        // VideoToolbox implementation is a stub for now
        // TODO: Implement VideoToolbox support
        Err(HardwareError::NotAvailable)
    }

    /// Check if a codec is supported by hardware
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cortenbrowser_hardware_accel::HardwareContext;
    /// use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let ctx = HardwareContext::new()?;
    ///
    /// let h264 = VideoCodec::H264 {
    ///     profile: H264Profile::High,
    ///     level: H264Level::Level4_1,
    ///     hardware_accel: true,
    /// };
    ///
    /// if ctx.is_codec_supported(&h264) {
    ///     println!("H.264 is hardware accelerated");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_codec_supported(&self, codec: &VideoCodec) -> bool {
        // Check if codec is in the supported list
        // For H.264, we match on codec type (ignoring specific profile/level)
        self.capabilities
            .supported_codecs
            .iter()
            .any(|supported| match (supported, codec) {
                (VideoCodec::H264 { .. }, VideoCodec::H264 { .. }) => true,
                (VideoCodec::VP9 { .. }, VideoCodec::VP9 { .. }) => true,
                (VideoCodec::VP8, VideoCodec::VP8) => true,
                (VideoCodec::H265 { .. }, VideoCodec::H265 { .. }) => true,
                (VideoCodec::AV1 { .. }, VideoCodec::AV1 { .. }) => true,
                _ => false,
            })
    }

    /// Create a hardware decoder for the specified codec
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `HardwareError::UnsupportedCodec` if the codec is not supported
    /// - `HardwareError::InitializationFailed` if decoder creation fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cortenbrowser_hardware_accel::HardwareContext;
    /// use cortenbrowser_shared_types::{VideoCodec, H264Profile, H264Level};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let ctx = HardwareContext::new()?;
    ///
    /// let h264 = VideoCodec::H264 {
    ///     profile: H264Profile::High,
    ///     level: H264Level::Level4_1,
    ///     hardware_accel: true,
    /// };
    ///
    /// let decoder = ctx.create_decoder(&h264)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_decoder(&self, codec: &VideoCodec) -> HardwareResult<Box<dyn VideoDecoder>> {
        // Check if codec is supported
        if !self.is_codec_supported(codec) {
            return Err(HardwareError::UnsupportedCodec);
        }

        // Create platform-specific decoder
        #[cfg(target_os = "linux")]
        {
            let decoder = VAAPIDecoder::new(codec)?;
            Ok(Box::new(decoder))
        }

        #[cfg(target_os = "windows")]
        {
            let decoder = DXVADecoder::new(codec)?;
            Ok(Box::new(decoder))
        }

        #[cfg(target_os = "macos")]
        {
            let decoder = VideoToolboxDecoder::new(codec)?;
            Ok(Box::new(decoder))
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Err(HardwareError::NotAvailable)
        }
    }

    /// Get hardware capabilities
    ///
    /// Returns information about supported codecs, maximum resolution,
    /// and frame rate.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cortenbrowser_hardware_accel::HardwareContext;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let ctx = HardwareContext::new()?;
    /// let caps = ctx.get_capabilities();
    ///
    /// println!("Max resolution: {}x{}", caps.max_resolution.0, caps.max_resolution.1);
    /// println!("Max framerate: {} fps", caps.max_framerate);
    /// println!("Supported codecs: {}", caps.supported_codecs.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_capabilities(&self) -> &HardwareCapabilities {
        &self.capabilities
    }
}
