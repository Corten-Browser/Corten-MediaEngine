//! Integration tests for hardware acceleration

use cortenbrowser_hardware_accel::{HardwareContext, HardwareError};
use cortenbrowser_shared_types::{H264Level, H264Profile, VideoCodec, VideoPacket};

#[test]
fn test_hardware_context_workflow() {
    /*
     * Given a system with potential hardware acceleration
     * When creating a hardware context
     * Then the context should be created or gracefully fail
     */
    let result = HardwareContext::new();

    match result {
        Ok(ctx) => {
            // Verify we can query capabilities
            let caps = ctx.get_capabilities();
            // Max resolution should be reasonable (not zero, not absurdly large)
            assert!(caps.max_resolution.0 > 0 && caps.max_resolution.0 <= 8192);
            assert!(caps.max_resolution.1 > 0 && caps.max_resolution.1 <= 8192);
        }
        Err(HardwareError::NotAvailable) => {
            // Acceptable - hardware not available
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_codec_support_detection() {
    /*
     * Given a hardware context
     * When checking codec support
     * Then should return consistent results
     */
    let result = HardwareContext::new();

    if let Ok(ctx) = result {
        let h264 = VideoCodec::H264 {
            profile: H264Profile::High,
            level: H264Level::Level4_1,
            hardware_accel: true,
        };

        let supported = ctx.is_codec_supported(&h264);

        // If codec is supported, we should be able to create a decoder
        if supported {
            let decoder_result = ctx.create_decoder(&h264);
            assert!(
                decoder_result.is_ok()
                    || matches!(decoder_result, Err(HardwareError::InitializationFailed))
            );
        }
    }
}

#[test]
fn test_fallback_behavior() {
    /*
     * Given a system without hardware acceleration
     * When attempting to create a decoder
     * Then should gracefully return error for software fallback
     */
    let result = HardwareContext::new();

    match result {
        Ok(ctx) => {
            let theora = VideoCodec::Theora; // Unlikely to be hardware accelerated

            let decoder_result = ctx.create_decoder(&theora);
            match decoder_result {
                Err(HardwareError::UnsupportedCodec) | Err(HardwareError::NotAvailable) => {
                    // Expected - software fallback needed
                }
                Ok(_) => {
                    // Theora hardware support is rare but acceptable
                }
                Err(e) => panic!("Unexpected error: {:?}", e),
            }
        }
        Err(HardwareError::NotAvailable) => {
            // Hardware not available - software fallback needed
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_decoder_lifecycle() {
    /*
     * Given a hardware decoder
     * When performing decode operations
     * Then should handle packets and produce frames
     */
    let result = HardwareContext::new();

    if let Ok(ctx) = result {
        let h264 = VideoCodec::H264 {
            profile: H264Profile::High,
            level: H264Level::Level4_1,
            hardware_accel: true,
        };

        if ctx.is_codec_supported(&h264) {
            if let Ok(mut decoder) = ctx.create_decoder(&h264) {
                // Create test packet
                let packet = VideoPacket {
                    data: vec![0u8; 100], // Mock H.264 data
                    pts: Some(0),
                    dts: Some(0),
                    is_keyframe: true,
                };

                // Decode should not panic (may fail due to invalid data)
                let _ = decoder.decode(&packet);

                // Flush should not panic
                let _ = decoder.flush();
            }
        }
    }
}

#[cfg(target_os = "linux")]
#[test]
fn test_linux_vaapi_availability() {
    /*
     * Given a Linux system
     * When creating hardware context
     * Then should attempt VA-API initialization
     */
    let result = HardwareContext::new();

    match result {
        Ok(_ctx) => {
            // VA-API available
            println!("VA-API hardware acceleration available");
        }
        Err(HardwareError::NotAvailable) => {
            // VA-API not available (no drivers or test environment)
            println!("VA-API not available - software fallback needed");
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[cfg(target_os = "windows")]
#[test]
fn test_windows_dxva_stub() {
    /*
     * Given a Windows system
     * When creating hardware context
     * Then should return NotAvailable (stub implementation)
     */
    let result = HardwareContext::new();

    // Windows stub should return NotAvailable
    assert!(matches!(result, Err(HardwareError::NotAvailable)));
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_videotoolbox_stub() {
    /*
     * Given a macOS system
     * When creating hardware context
     * Then should return NotAvailable (stub implementation)
     */
    let result = HardwareContext::new();

    // macOS stub should return NotAvailable
    assert!(matches!(result, Err(HardwareError::NotAvailable)));
}
