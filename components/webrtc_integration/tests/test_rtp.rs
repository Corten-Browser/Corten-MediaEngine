//! Unit tests for RTP packet handling
//!
//! Tests for RTPPacket struct and RTPPacketizer

#[cfg(test)]
mod tests {
    use cortenbrowser_webrtc_integration::{RTPPacket, RTPPacketizer};

    #[test]
    fn test_rtp_packet_creation() {
        // RED: Test RTPPacket creation with all fields
        let packet = RTPPacket {
            payload: vec![1, 2, 3, 4, 5],
            sequence_number: 100,
            timestamp: 1000,
            ssrc: 0x12345678,
        };

        assert_eq!(packet.payload, vec![1, 2, 3, 4, 5]);
        assert_eq!(packet.sequence_number, 100);
        assert_eq!(packet.timestamp, 1000);
        assert_eq!(packet.ssrc, 0x12345678);
    }

    #[test]
    fn test_rtp_packet_serialization() {
        // RED: Test RTPPacket serialization to bytes
        let packet = RTPPacket {
            payload: vec![0xAA, 0xBB, 0xCC],
            sequence_number: 42,
            timestamp: 9000,
            ssrc: 0xDEADBEEF,
        };

        let bytes = packet.to_bytes();

        // RTP header format (simplified, fixed header is 12 bytes):
        // Version (2 bits) = 2, P/X/CC/M/PT fields (14 bits)
        // Sequence Number (16 bits)
        // Timestamp (32 bits)
        // SSRC (32 bits)
        // Payload (variable)

        // Minimum header is 12 bytes + payload
        assert!(bytes.len() >= 12 + 3, "Header should be at least 12 bytes + payload");

        // Check sequence number (bytes 2-3)
        let seq = u16::from_be_bytes([bytes[2], bytes[3]]);
        assert_eq!(seq, 42);

        // Check timestamp (bytes 4-7)
        let ts = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        assert_eq!(ts, 9000);

        // Check SSRC (bytes 8-11)
        let ssrc = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        assert_eq!(ssrc, 0xDEADBEEF);

        // Check payload starts at byte 12
        assert_eq!(&bytes[12..], &[0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn test_rtp_packetizer_single_packet() {
        // RED: Test packetizing small payload into single RTP packet
        let packetizer = RTPPacketizer::new();
        let payload = vec![1, 2, 3, 4, 5];
        let timestamp = 3000;

        let packets = packetizer.packetize(&payload, timestamp);

        assert_eq!(packets.len(), 1, "Small payload should fit in one packet");
        assert_eq!(packets[0].payload, payload);
        assert_eq!(packets[0].timestamp, timestamp);
        assert_eq!(packets[0].sequence_number, 0, "First packet should have sequence 0");
    }

    #[test]
    fn test_rtp_packetizer_fragmentation() {
        // RED: Test packetizing large payload into multiple RTP packets
        let packetizer = RTPPacketizer::new();

        // Create payload larger than MTU (Maximum Transmission Unit ~1200 bytes)
        let large_payload = vec![0x42; 3000];
        let timestamp = 5000;

        let packets = packetizer.packetize(&large_payload, timestamp);

        // Should be fragmented into multiple packets
        assert!(packets.len() > 1, "Large payload should be fragmented");

        // All packets should have same timestamp
        for packet in &packets {
            assert_eq!(packet.timestamp, timestamp);
        }

        // Sequence numbers should increment
        for i in 0..packets.len() {
            assert_eq!(packets[i].sequence_number, i as u16);
        }

        // Reassemble payload to verify
        let reassembled: Vec<u8> = packets.iter()
            .flat_map(|p| p.payload.clone())
            .collect();
        assert_eq!(reassembled, large_payload);
    }

    #[test]
    fn test_rtp_packetizer_sequence_wraparound() {
        // RED: Test sequence number wraparound at u16::MAX
        let mut packetizer = RTPPacketizer::new();
        packetizer.set_sequence_number(65534); // Near max

        let payload = vec![1, 2, 3];

        let packets1 = packetizer.packetize(&payload, 1000);
        assert_eq!(packets1[0].sequence_number, 65534);

        let packets2 = packetizer.packetize(&payload, 1100);
        assert_eq!(packets2[0].sequence_number, 65535);

        let packets3 = packetizer.packetize(&payload, 1200);
        assert_eq!(packets3[0].sequence_number, 0, "Should wrap to 0");
    }

    #[test]
    fn test_rtp_packetizer_ssrc_assignment() {
        // RED: Test that SSRC is assigned and consistent
        let packetizer = RTPPacketizer::new();
        let payload = vec![1, 2, 3, 4];

        let packets1 = packetizer.packetize(&payload, 1000);
        let packets2 = packetizer.packetize(&payload, 2000);

        assert_eq!(packets1[0].ssrc, packets2[0].ssrc, "SSRC should be consistent");
        assert_ne!(packets1[0].ssrc, 0, "SSRC should not be zero");
    }
}
