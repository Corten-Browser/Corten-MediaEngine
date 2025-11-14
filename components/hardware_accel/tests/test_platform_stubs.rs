//! Unit tests for platform-specific stubs (DXVA, VideoToolbox)

#[cfg(target_os = "windows")]
mod dxva_tests {
    use cortenbrowser_hardware_accel::{DXVADecoder, HardwareError};
    use cortenbrowser_shared_types::{H264Level, H264Profile, VideoCodec};

    #[test]
    fn test_dxva_decoder_new_returns_not_implemented() {
        let codec = VideoCodec::H264 {
            profile: H264Profile::High,
            level: H264Level::Level4_1,
            hardware_accel: true,
        };

        let result = DXVADecoder::new(&codec);

        // Stub should return NotAvailable (not yet implemented)
        assert!(matches!(result, Err(HardwareError::NotAvailable)));
    }
}

#[cfg(target_os = "macos")]
mod videotoolbox_tests {
    use cortenbrowser_hardware_accel::{HardwareError, VideoToolboxDecoder};
    use cortenbrowser_shared_types::{H264Level, H264Profile, VideoCodec};

    #[test]
    fn test_videotoolbox_decoder_new_returns_not_implemented() {
        let codec = VideoCodec::H264 {
            profile: H264Profile::High,
            level: H264Level::Level4_1,
            hardware_accel: true,
        };

        let result = VideoToolboxDecoder::new(&codec);

        // Stub should return NotAvailable (not yet implemented)
        assert!(matches!(result, Err(HardwareError::NotAvailable)));
    }
}
