//! Integration tests for WebRTC packet flow
//!
//! Tests the complete flow: encoding -> packetization -> jitter buffer

use cortenbrowser_webrtc_integration::{
    WebRTCEncoder, EncoderConfig, RTPPacketizer, JitterBuffer,
};
use cortenbrowser_shared_types::{
    VideoCodec, VideoFrame, PixelFormat, H264Profile, H264Level, FrameMetadata,
};
use std::time::Duration;

#[test]
fn test_complete_packet_flow() {
    // Step 1: Create encoder
    let codec = VideoCodec::H264 {
        profile: H264Profile::Main,
        level: H264Level::Level4_0,
        hardware_accel: false,
    };

    let config = EncoderConfig {
        bitrate: 1_000_000,
        framerate: 30,
        keyframe_interval: 30,
    };

    let encoder = WebRTCEncoder::new(codec, config).expect("Failed to create encoder");

    // Step 2: Create frame and encode
    let frame = VideoFrame {
        width: 640,
        height: 480,
        format: PixelFormat::YUV420,
        data: vec![42u8; 640 * 480 * 3 / 2],
        timestamp: Duration::from_millis(0),
        duration: Some(Duration::from_millis(33)),
        metadata: FrameMetadata::default(),
    };

    let encoded = encoder.encode(&frame).expect("Failed to encode frame");
    assert!(!encoded.is_empty(), "Encoded data should not be empty");

    // Step 3: Packetize encoded data
    let packetizer = RTPPacketizer::new();
    let timestamp = 3000;
    let packets = packetizer.packetize(&encoded, timestamp);

    assert!(!packets.is_empty(), "Should produce at least one packet");
    assert_eq!(packets[0].timestamp, timestamp, "Timestamp should match");

    // Step 4: Insert packets into jitter buffer (simulate out-of-order arrival)
    let mut jitter_buffer = JitterBuffer::new(100);

    // Insert in reverse order to test reordering
    for packet in packets.iter().rev() {
        jitter_buffer
            .insert(packet.clone())
            .expect("Failed to insert packet");
    }

    // Step 5: Retrieve packets in order
    for i in 0..packets.len() {
        let retrieved = jitter_buffer
            .get_next()
            .expect("Should retrieve packet");
        assert_eq!(
            retrieved.sequence_number, i as u16,
            "Packets should be in sequence order"
        );
        assert_eq!(retrieved.timestamp, timestamp);
    }

    // Buffer should be empty
    assert_eq!(jitter_buffer.get_next(), None, "Buffer should be empty");
}

#[test]
fn test_multiple_frame_flow() {
    // Encode and transmit multiple frames
    let encoder = WebRTCEncoder::new(
        VideoCodec::VP8,
        EncoderConfig {
            bitrate: 1_000_000,
            framerate: 30,
            keyframe_interval: 10,
        },
    )
    .unwrap();

    let packetizer = RTPPacketizer::new();
    let mut jitter_buffer = JitterBuffer::new(200);

    let num_frames = 5;

    // Encode and packetize multiple frames
    for frame_idx in 0..num_frames {
        let frame = VideoFrame {
            width: 320,
            height: 240,
            format: PixelFormat::YUV420,
            data: vec![(frame_idx * 10) as u8; 320 * 240 * 3 / 2],
            timestamp: Duration::from_millis(frame_idx * 33),
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata {
                is_keyframe: frame_idx == 0,
                ..Default::default()
            },
        };

        let encoded = encoder.encode(&frame).unwrap();
        let timestamp = (frame_idx * 3000) as u32;
        let packets = packetizer.packetize(&encoded, timestamp);

        // Insert all packets
        for packet in packets {
            jitter_buffer.insert(packet).unwrap();
        }
    }

    // Retrieve all packets - should be in sequence order
    let mut last_seq: Option<u16> = None;
    let mut packet_count = 0;

    while let Some(packet) = jitter_buffer.get_next() {
        if let Some(last) = last_seq {
            assert_eq!(
                packet.sequence_number,
                last.wrapping_add(1),
                "Sequence numbers should increment"
            );
        }
        last_seq = Some(packet.sequence_number);
        packet_count += 1;
    }

    assert!(packet_count > 0, "Should have retrieved packets");
}

#[test]
fn test_packet_loss_handling() {
    // Simulate packet loss scenario
    let packetizer = RTPPacketizer::new();
    let mut jitter_buffer = JitterBuffer::new(50);

    // Create large payload that will fragment
    let large_payload = vec![0x55; 5000];
    let packets = packetizer.packetize(&large_payload, 9000);

    assert!(packets.len() > 2, "Should have multiple packets");

    // Insert all packets except one in the middle
    let skip_index = packets.len() / 2;

    for (i, packet) in packets.iter().enumerate() {
        if i != skip_index {
            jitter_buffer.insert(packet.clone()).unwrap();
        }
    }

    // Should retrieve packets up to the gap
    for i in 0..skip_index {
        let retrieved = jitter_buffer.get_next().expect("Should retrieve packet");
        assert_eq!(retrieved.sequence_number, i as u16);
    }

    // Should stop at gap (waiting for missing packet)
    assert_eq!(
        jitter_buffer.get_next(),
        None,
        "Should wait for missing packet"
    );

    // Late arrival of missing packet
    jitter_buffer
        .insert(packets[skip_index].clone())
        .unwrap();

    // Now should be able to retrieve rest
    for i in skip_index..packets.len() {
        let retrieved = jitter_buffer.get_next().expect("Should retrieve packet");
        assert_eq!(retrieved.sequence_number, i as u16);
    }
}

#[test]
fn test_different_codecs() {
    // Test with different codecs
    let codecs = vec![
        VideoCodec::H264 {
            profile: H264Profile::Main,
            level: H264Level::Level4_0,
            hardware_accel: false,
        },
        VideoCodec::VP8,
        VideoCodec::VP9 {
            profile: cortenbrowser_shared_types::VP9Profile::Profile0,
        },
    ];

    for codec in codecs {
        let encoder = WebRTCEncoder::new(
            codec,
            EncoderConfig {
                bitrate: 1_000_000,
                framerate: 30,
                keyframe_interval: 30,
            },
        )
        .expect("Should create encoder");

        let frame = VideoFrame {
            width: 640,
            height: 480,
            format: PixelFormat::YUV420,
            data: vec![0u8; 640 * 480 * 3 / 2],
            timestamp: Duration::from_millis(0),
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata::default(),
        };

        let encoded = encoder.encode(&frame).expect("Should encode frame");
        assert!(!encoded.is_empty());

        let packetizer = RTPPacketizer::new();
        let packets = packetizer.packetize(&encoded, 1000);
        assert!(!packets.is_empty());
    }
}

#[test]
fn test_high_throughput() {
    // Test handling high packet rate with realistic consumption pattern
    let encoder = WebRTCEncoder::new(
        VideoCodec::H264 {
            profile: H264Profile::High,
            level: H264Level::Level4_1,
            hardware_accel: false,
        },
        EncoderConfig {
            bitrate: 2_000_000, // 2 Mbps
            framerate: 30,
            keyframe_interval: 30,
        },
    )
    .unwrap();

    let packetizer = RTPPacketizer::new();
    let mut jitter_buffer = JitterBuffer::new(500); // Buffer size to handle multiple frames
    let mut total_packets_processed = 0;

    // Simulate 1 second at 30fps with realistic consumption pattern
    // (insert packets, periodically consume them like a real decoder would)
    for i in 0..30 {
        let frame = VideoFrame {
            width: 1280,
            height: 720,
            format: PixelFormat::YUV420,
            data: vec![i as u8; 1280 * 720 * 3 / 2],
            timestamp: Duration::from_millis(i * 33), // ~30fps
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata::default(),
        };

        let encoded = encoder.encode(&frame).unwrap();
        let packets = packetizer.packetize(&encoded, (i * 3000) as u32); // Timestamp increment

        // Insert packets
        for packet in packets {
            jitter_buffer.insert(packet).unwrap();
        }

        // Consume packets periodically (simulate decoder pulling packets)
        // This prevents buffer overflow and mimics real-world usage
        while let Some(_packet) = jitter_buffer.get_next() {
            total_packets_processed += 1;
        }
    }

    // Retrieve any remaining packets
    while let Some(_packet) = jitter_buffer.get_next() {
        total_packets_processed += 1;
    }

    assert!(
        total_packets_processed > 0,
        "Should have processed packets (got {})",
        total_packets_processed
    );

    // For 30 frames at 720p with 2Mbps, we expect many packets
    assert!(
        total_packets_processed >= 30,
        "Should have at least one packet per frame (got {})",
        total_packets_processed
    );
}
