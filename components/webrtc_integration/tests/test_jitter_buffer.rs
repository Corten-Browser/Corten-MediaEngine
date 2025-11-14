//! Unit tests for jitter buffer
//!
//! Tests for JitterBuffer packet reordering and buffering

#[cfg(test)]
mod tests {
    use cortenbrowser_webrtc_integration::{JitterBuffer, RTPPacket, MediaError};

    #[test]
    fn test_jitter_buffer_creation() {
        // RED: Test JitterBuffer creation
        let buffer = JitterBuffer::new(100);

        // Should be empty initially
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 100);
    }

    #[test]
    fn test_jitter_buffer_insert_and_retrieve_ordered() {
        // RED: Test inserting packets in order
        let mut buffer = JitterBuffer::new(10);

        let packet1 = RTPPacket {
            payload: vec![1],
            sequence_number: 0,
            timestamp: 1000,
            ssrc: 12345,
        };

        let packet2 = RTPPacket {
            payload: vec![2],
            sequence_number: 1,
            timestamp: 1100,
            ssrc: 12345,
        };

        buffer.insert(packet1).expect("Insert should succeed");
        buffer.insert(packet2).expect("Insert should succeed");

        assert_eq!(buffer.len(), 2);

        let retrieved1 = buffer.get_next().expect("Should return packet");
        assert_eq!(retrieved1.sequence_number, 0);
        assert_eq!(retrieved1.payload, vec![1]);

        let retrieved2 = buffer.get_next().expect("Should return packet");
        assert_eq!(retrieved2.sequence_number, 1);
        assert_eq!(retrieved2.payload, vec![2]);

        assert_eq!(buffer.get_next(), None, "Buffer should be empty");
    }

    #[test]
    fn test_jitter_buffer_reordering() {
        // RED: Test inserting packets out of order
        let mut buffer = JitterBuffer::new(10);

        // Insert in reverse order
        let packet2 = RTPPacket {
            payload: vec![2],
            sequence_number: 2,
            timestamp: 1200,
            ssrc: 12345,
        };

        let packet1 = RTPPacket {
            payload: vec![1],
            sequence_number: 1,
            timestamp: 1100,
        ssrc: 12345,
        };

        let packet0 = RTPPacket {
            payload: vec![0],
            sequence_number: 0,
            timestamp: 1000,
            ssrc: 12345,
        };

        buffer.insert(packet2).unwrap();
        buffer.insert(packet1).unwrap();
        buffer.insert(packet0).unwrap();

        assert_eq!(buffer.len(), 3);

        // Should retrieve in correct sequence order
        assert_eq!(buffer.get_next().unwrap().sequence_number, 0);
        assert_eq!(buffer.get_next().unwrap().sequence_number, 1);
        assert_eq!(buffer.get_next().unwrap().sequence_number, 2);
    }

    #[test]
    fn test_jitter_buffer_sequence_wraparound() {
        // RED: Test handling sequence number wraparound
        let mut buffer = JitterBuffer::new(10);

        let packet_max = RTPPacket {
            payload: vec![255],
            sequence_number: 65535, // u16::MAX
            timestamp: 1000,
            ssrc: 12345,
        };

        let packet_zero = RTPPacket {
            payload: vec![0],
            sequence_number: 0, // Wraps around
            timestamp: 1100,
            ssrc: 12345,
        };

        let packet_one = RTPPacket {
            payload: vec![1],
            sequence_number: 1,
            timestamp: 1200,
            ssrc: 12345,
        };

        buffer.insert(packet_zero).unwrap();
        buffer.insert(packet_max).unwrap();
        buffer.insert(packet_one).unwrap();

        // Should retrieve in wrapped order: 65535, 0, 1
        assert_eq!(buffer.get_next().unwrap().sequence_number, 65535);
        assert_eq!(buffer.get_next().unwrap().sequence_number, 0);
        assert_eq!(buffer.get_next().unwrap().sequence_number, 1);
    }

    #[test]
    fn test_jitter_buffer_duplicate_packet() {
        // RED: Test handling duplicate packets (same sequence number)
        let mut buffer = JitterBuffer::new(10);

        let packet1 = RTPPacket {
            payload: vec![1],
            sequence_number: 5,
            timestamp: 1000,
            ssrc: 12345,
        };

        let packet1_dup = RTPPacket {
            payload: vec![1, 2, 3], // Different payload
            sequence_number: 5, // Same sequence
            timestamp: 1000,
            ssrc: 12345,
        };

        buffer.insert(packet1.clone()).unwrap();

        // Duplicate should be silently dropped
        buffer.insert(packet1_dup).unwrap();

        assert_eq!(buffer.len(), 1, "Duplicate should be dropped");

        let retrieved = buffer.get_next().unwrap();
        assert_eq!(retrieved.payload, vec![1], "Original packet should remain");
    }

    #[test]
    fn test_jitter_buffer_capacity_exceeded() {
        // RED: Test buffer behavior when capacity is exceeded
        let mut buffer = JitterBuffer::new(3);

        for i in 0..5 {
            let packet = RTPPacket {
                payload: vec![i],
                sequence_number: i as u16,
                timestamp: 1000 + i as u32 * 100,
                ssrc: 12345,
            };

            let result = buffer.insert(packet);

            if i < 3 {
                assert!(result.is_ok(), "First 3 inserts should succeed");
            } else {
                // Behavior when full: could drop oldest or return error
                // For now, expect error
                assert!(result.is_err(), "Should error when capacity exceeded");
            }
        }
    }

    #[test]
    fn test_jitter_buffer_empty() {
        // RED: Test get_next on empty buffer
        let mut buffer = JitterBuffer::new(10);

        assert_eq!(buffer.get_next(), None, "Empty buffer should return None");
    }

    #[test]
    fn test_jitter_buffer_gap_detection() {
        // RED: Test detection of sequence number gaps
        let mut buffer = JitterBuffer::new(10);

        let packet0 = RTPPacket {
            payload: vec![0],
            sequence_number: 0,
            timestamp: 1000,
            ssrc: 12345,
        };

        let packet2 = RTPPacket {
            payload: vec![2],
            sequence_number: 2, // Gap: missing seq 1
            timestamp: 1200,
            ssrc: 12345,
        };

        buffer.insert(packet0).unwrap();
        buffer.insert(packet2).unwrap();

        // Should only return packet 0, hold packet 2 waiting for 1
        let retrieved = buffer.get_next().unwrap();
        assert_eq!(retrieved.sequence_number, 0);

        // Packet 2 should still be in buffer
        assert_eq!(buffer.get_next(), None, "Should wait for missing packet");

        // Now insert missing packet
        let packet1 = RTPPacket {
            payload: vec![1],
            sequence_number: 1,
            timestamp: 1100,
            ssrc: 12345,
        };

        buffer.insert(packet1).unwrap();

        // Now should be able to retrieve both
        assert_eq!(buffer.get_next().unwrap().sequence_number, 1);
        assert_eq!(buffer.get_next().unwrap().sequence_number, 2);
    }
}
