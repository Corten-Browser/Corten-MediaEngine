//! Unit tests for codec enums

use cortenbrowser_shared_types::{
    AACProfile, AV1Level, AV1Profile, AudioCodec, H264Level, H264Profile, H265Level, H265Profile,
    H265Tier, MP3Layer, OpusApplication, PCMFormat, VP9Profile, VideoCodec,
};

#[test]
fn test_video_codec_h264_creation() {
    let codec = VideoCodec::H264 {
        profile: H264Profile::Baseline,
        level: H264Level::Level4_1,
        hardware_accel: true,
    };

    match codec {
        VideoCodec::H264 {
            profile,
            level,
            hardware_accel,
        } => {
            assert_eq!(profile, H264Profile::Baseline);
            assert_eq!(level, H264Level::Level4_1);
            assert!(hardware_accel);
        }
        _ => panic!("Expected H264 codec"),
    }
}

#[test]
fn test_video_codec_vp8() {
    let codec = VideoCodec::VP8;
    assert!(matches!(codec, VideoCodec::VP8));
}

#[test]
fn test_video_codec_av1() {
    let codec = VideoCodec::AV1 {
        profile: AV1Profile::Main,
        level: AV1Level::Level4_0,
    };

    match codec {
        VideoCodec::AV1 { profile, level } => {
            assert_eq!(profile, AV1Profile::Main);
            assert_eq!(level, AV1Level::Level4_0);
        }
        _ => panic!("Expected AV1 codec"),
    }
}

#[test]
fn test_audio_codec_aac() {
    let codec = AudioCodec::AAC {
        profile: AACProfile::LC,
        sample_rate: 48000,
        channels: 2,
    };

    match codec {
        AudioCodec::AAC {
            profile,
            sample_rate,
            channels,
        } => {
            assert_eq!(profile, AACProfile::LC);
            assert_eq!(sample_rate, 48000);
            assert_eq!(channels, 2);
        }
        _ => panic!("Expected AAC codec"),
    }
}

#[test]
fn test_audio_codec_opus() {
    let codec = AudioCodec::Opus {
        sample_rate: 48000,
        channels: 2,
        application: OpusApplication::Audio,
    };

    match codec {
        AudioCodec::Opus {
            sample_rate,
            channels,
            application,
        } => {
            assert_eq!(sample_rate, 48000);
            assert_eq!(channels, 2);
            assert_eq!(application, OpusApplication::Audio);
        }
        _ => panic!("Expected Opus codec"),
    }
}

#[test]
fn test_audio_codec_pcm() {
    let codec = AudioCodec::PCM {
        format: PCMFormat::F32LE,
        sample_rate: 44100,
        channels: 2,
    };

    match codec {
        AudioCodec::PCM {
            format,
            sample_rate,
            channels,
        } => {
            assert_eq!(format, PCMFormat::F32LE);
            assert_eq!(sample_rate, 44100);
            assert_eq!(channels, 2);
        }
        _ => panic!("Expected PCM codec"),
    }
}

#[test]
fn test_video_codec_debug() {
    let codec = VideoCodec::VP9 {
        profile: VP9Profile::Profile0,
    };
    let debug_str = format!("{:?}", codec);
    assert!(debug_str.contains("VP9"));
}

#[test]
fn test_audio_codec_debug() {
    let codec = AudioCodec::MP3 {
        layer: MP3Layer::Layer3,
        bitrate: 320000,
    };
    let debug_str = format!("{:?}", codec);
    assert!(debug_str.contains("MP3"));
}

#[test]
fn test_video_codec_clone() {
    let codec1 = VideoCodec::Theora;
    let codec2 = codec1.clone();
    assert!(matches!(codec2, VideoCodec::Theora));
}

#[test]
fn test_audio_codec_clone() {
    let codec1 = AudioCodec::Vorbis;
    let codec2 = codec1.clone();
    assert!(matches!(codec2, AudioCodec::Vorbis));
}
