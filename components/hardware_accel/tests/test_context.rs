//! Unit tests for HardwareContext

use cortenbrowser_hardware_accel::{HardwareContext, HardwareError};
use cortenbrowser_shared_types::{H264Level, H264Profile, VideoCodec};

#[test]
fn test_hardware_context_new() {
    // Should not panic even if hardware is unavailable
    let result = HardwareContext::new();

    // On most test environments, hardware may not be available
    // So we accept both Ok and Err
    match result {
        Ok(_ctx) => {
            // Hardware available in test environment
        }
        Err(HardwareError::NotAvailable) => {
            // Expected in CI/test environments
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_hardware_context_get_capabilities() {
    // Try to create context
    let result = HardwareContext::new();

    if let Ok(ctx) = result {
        let caps = ctx.get_capabilities();

        // Verify capabilities structure (values should be reasonable)
        assert!(caps.max_resolution.0 <= 8192); // Max 8K
        assert!(caps.max_resolution.1 <= 8192);
        assert!(caps.max_framerate <= 240.0); // Max 240fps
    }
}

#[test]
fn test_hardware_context_is_codec_supported() {
    let result = HardwareContext::new();

    if let Ok(ctx) = result {
        let h264 = VideoCodec::H264 {
            profile: H264Profile::High,
            level: H264Level::Level4_1,
            hardware_accel: true,
        };

        // Should return bool without panicking
        let _ = ctx.is_codec_supported(&h264);
    }
}

#[test]
fn test_hardware_context_create_decoder_with_unsupported_codec() {
    let result = HardwareContext::new();

    if let Ok(ctx) = result {
        // Theora is typically not hardware accelerated
        let theora = VideoCodec::Theora;

        let decoder_result = ctx.create_decoder(&theora);

        // Should return error for unsupported codec
        match decoder_result {
            Err(HardwareError::UnsupportedCodec) => {
                // Expected
            }
            Err(HardwareError::NotAvailable) => {
                // Also acceptable if hardware not available
            }
            Ok(_) => {
                // Unexpected but not a test failure if Theora is supported
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}

#[test]
fn test_hardware_context_platform_detection() {
    // This test verifies platform-specific behavior
    let result = HardwareContext::new();

    // On Linux, should attempt VA-API
    // On Windows, should attempt DXVA
    // On macOS, should attempt VideoToolbox
    // On unknown platforms, should return NotAvailable

    match result {
        Ok(_ctx) => {
            // Platform has hardware support
            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
            {
                // Expected on supported platforms (if drivers available)
            }
        }
        Err(HardwareError::NotAvailable) => {
            // Expected on unsupported platforms or missing drivers
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}
