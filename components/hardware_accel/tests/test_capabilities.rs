//! Unit tests for HardwareCapabilities struct

use cortenbrowser_hardware_accel::HardwareCapabilities;
use cortenbrowser_shared_types::{H264Level, H264Profile, VP9Profile, VideoCodec};

#[test]
fn test_hardware_capabilities_creation() {
    let caps = HardwareCapabilities {
        supported_codecs: vec![
            VideoCodec::H264 {
                profile: H264Profile::High,
                level: H264Level::Level4_1,
                hardware_accel: true,
            },
            VideoCodec::VP9 {
                profile: VP9Profile::Profile0,
            },
        ],
        max_resolution: (3840, 2160), // 4K
        max_framerate: 60.0,
    };

    assert_eq!(caps.supported_codecs.len(), 2);
    assert_eq!(caps.max_resolution, (3840, 2160));
    assert_eq!(caps.max_framerate, 60.0);
}

#[test]
fn test_hardware_capabilities_empty() {
    let caps = HardwareCapabilities {
        supported_codecs: vec![],
        max_resolution: (0, 0),
        max_framerate: 0.0,
    };

    assert_eq!(caps.supported_codecs.len(), 0);
    assert_eq!(caps.max_resolution, (0, 0));
}

#[test]
fn test_hardware_capabilities_debug_format() {
    let caps = HardwareCapabilities {
        supported_codecs: vec![],
        max_resolution: (1920, 1080),
        max_framerate: 30.0,
    };

    let debug_str = format!("{:?}", caps);
    assert!(debug_str.contains("HardwareCapabilities"));
}

#[test]
fn test_hardware_capabilities_clone() {
    let caps = HardwareCapabilities {
        supported_codecs: vec![VideoCodec::VP8],
        max_resolution: (1920, 1080),
        max_framerate: 30.0,
    };

    let cloned = caps.clone();
    assert_eq!(caps.supported_codecs.len(), cloned.supported_codecs.len());
    assert_eq!(caps.max_resolution, cloned.max_resolution);
    assert_eq!(caps.max_framerate, cloned.max_framerate);
}
