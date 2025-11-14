//! Jitter buffer for RTP packet reordering
//!
//! Handles out-of-order packet arrival and sequence number wraparound.

use crate::rtp::RTPPacket;
use cortenbrowser_shared_types::MediaError;
use std::collections::HashMap;

/// Jitter buffer for reordering RTP packets
///
/// Stores packets and returns them in sequence number order.
/// Handles sequence number wraparound (u16::MAX -> 0).
///
/// # Examples
///
/// ```
/// use cortenbrowser_webrtc_integration::{JitterBuffer, RTPPacket};
///
/// let mut buffer = JitterBuffer::new(100);
///
/// let packet = RTPPacket {
///     payload: vec![1, 2, 3],
///     sequence_number: 5,
///     timestamp: 1000,
///     ssrc: 12345,
/// };
///
/// buffer.insert(packet).unwrap();
/// let retrieved = buffer.get_next().unwrap();
/// assert_eq!(retrieved.sequence_number, 5);
/// ```
pub struct JitterBuffer {
    capacity: usize,
    packets: HashMap<u16, RTPPacket>,
    next_expected_seq: Option<u16>,
}

impl JitterBuffer {
    /// Create a new jitter buffer with specified capacity
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of packets to buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_webrtc_integration::JitterBuffer;
    ///
    /// let buffer = JitterBuffer::new(100);
    /// assert_eq!(buffer.capacity(), 100);
    /// assert_eq!(buffer.len(), 0);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            packets: HashMap::new(),
            next_expected_seq: None,
        }
    }

    /// Get the buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the number of packets currently in the buffer
    pub fn len(&self) -> usize {
        self.packets.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }

    /// Insert a packet into the buffer
    ///
    /// Handles duplicates by keeping the first packet received.
    ///
    /// # Arguments
    ///
    /// * `packet` - The RTP packet to insert
    ///
    /// # Errors
    ///
    /// Returns `MediaError::OutOfMemory` if buffer is at capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_webrtc_integration::{JitterBuffer, RTPPacket};
    ///
    /// let mut buffer = JitterBuffer::new(10);
    /// let packet = RTPPacket {
    ///     payload: vec![1, 2, 3],
    ///     sequence_number: 0,
    ///     timestamp: 1000,
    ///     ssrc: 12345,
    /// };
    ///
    /// assert!(buffer.insert(packet).is_ok());
    /// ```
    pub fn insert(&mut self, packet: RTPPacket) -> Result<(), MediaError> {
        // Save sequence number before move
        let seq = packet.sequence_number;

        // Check capacity (exclude duplicates from count)
        if self.packets.len() >= self.capacity && !self.packets.contains_key(&seq) {
            return Err(MediaError::OutOfMemory);
        }

        // Insert packet (duplicates are kept as first)
        // But we only insert if not already present
        self.packets.entry(seq).or_insert(packet);

        // Update expected sequence
        if self.next_expected_seq.is_none() {
            // First packet sets the starting point
            self.next_expected_seq = Some(seq);
        } else {
            // If this packet is before our expected start, update start
            let expected = self.next_expected_seq.unwrap();
            if Self::sequence_before(seq, expected) {
                self.next_expected_seq = Some(seq);
            }
        }

        Ok(())
    }

    /// Get the next packet in sequence order
    ///
    /// Returns `None` if buffer is empty or if the next expected packet
    /// hasn't arrived yet (gap detected).
    ///
    /// # Returns
    ///
    /// The next packet in sequence, or `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_webrtc_integration::{JitterBuffer, RTPPacket};
    ///
    /// let mut buffer = JitterBuffer::new(10);
    ///
    /// let packet1 = RTPPacket {
    ///     payload: vec![1],
    ///     sequence_number: 0,
    ///     timestamp: 1000,
    ///     ssrc: 12345,
    /// };
    ///
    /// let packet2 = RTPPacket {
    ///     payload: vec![2],
    ///     sequence_number: 1,
    ///     timestamp: 1100,
    ///     ssrc: 12345,
    /// };
    ///
    /// buffer.insert(packet1).unwrap();
    /// buffer.insert(packet2).unwrap();
    ///
    /// assert_eq!(buffer.get_next().unwrap().sequence_number, 0);
    /// assert_eq!(buffer.get_next().unwrap().sequence_number, 1);
    /// assert_eq!(buffer.get_next(), None);
    /// ```
    pub fn get_next(&mut self) -> Option<RTPPacket> {
        if let Some(expected_seq) = self.next_expected_seq {
            if let Some(packet) = self.packets.remove(&expected_seq) {
                // Move to next expected sequence (with wraparound)
                self.next_expected_seq = Some(expected_seq.wrapping_add(1));
                return Some(packet);
            }
        }

        None
    }

    /// Helper function to check if sequence a comes before sequence b
    /// considering wraparound
    fn sequence_before(a: u16, b: u16) -> bool {
        // Handle wraparound: difference should be positive in modular arithmetic
        let diff = b.wrapping_sub(a);
        diff > 0 && diff < 32768 // Half of u16 range
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitter_buffer_reordering() {
        let mut buffer = JitterBuffer::new(10);

        // Insert out of order
        buffer.insert(RTPPacket {
            payload: vec![2],
            sequence_number: 2,
            timestamp: 1200,
            ssrc: 12345,
        }).unwrap();

        buffer.insert(RTPPacket {
            payload: vec![0],
            sequence_number: 0,
            timestamp: 1000,
            ssrc: 12345,
        }).unwrap();

        buffer.insert(RTPPacket {
            payload: vec![1],
            sequence_number: 1,
            timestamp: 1100,
            ssrc: 12345,
        }).unwrap();

        // Should retrieve in order
        assert_eq!(buffer.get_next().unwrap().sequence_number, 0);
        assert_eq!(buffer.get_next().unwrap().sequence_number, 1);
        assert_eq!(buffer.get_next().unwrap().sequence_number, 2);
        assert_eq!(buffer.get_next(), None);
    }

    #[test]
    fn test_jitter_buffer_gap_handling() {
        let mut buffer = JitterBuffer::new(10);

        // Insert packet 0
        buffer.insert(RTPPacket {
            payload: vec![0],
            sequence_number: 0,
            timestamp: 1000,
            ssrc: 12345,
        }).unwrap();

        // Insert packet 2 (gap at 1)
        buffer.insert(RTPPacket {
            payload: vec![2],
            sequence_number: 2,
            timestamp: 1200,
            ssrc: 12345,
        }).unwrap();

        // Should return packet 0
        assert_eq!(buffer.get_next().unwrap().sequence_number, 0);

        // Should NOT return packet 2 (waiting for 1)
        assert_eq!(buffer.get_next(), None);

        // Insert missing packet
        buffer.insert(RTPPacket {
            payload: vec![1],
            sequence_number: 1,
            timestamp: 1100,
            ssrc: 12345,
        }).unwrap();

        // Now should return 1 and 2
        assert_eq!(buffer.get_next().unwrap().sequence_number, 1);
        assert_eq!(buffer.get_next().unwrap().sequence_number, 2);
    }

    #[test]
    fn test_jitter_buffer_wraparound() {
        let mut buffer = JitterBuffer::new(10);

        // Insert near u16::MAX
        buffer.insert(RTPPacket {
            payload: vec![255, 255],
            sequence_number: 65535,
            timestamp: 1000,
            ssrc: 12345,
        }).unwrap();

        buffer.insert(RTPPacket {
            payload: vec![0, 0],
            sequence_number: 0,
            timestamp: 1100,
            ssrc: 12345,
        }).unwrap();

        buffer.insert(RTPPacket {
            payload: vec![0, 1],
            sequence_number: 1,
            timestamp: 1200,
            ssrc: 12345,
        }).unwrap();

        // Should retrieve in wrapped order
        assert_eq!(buffer.get_next().unwrap().sequence_number, 65535);
        assert_eq!(buffer.get_next().unwrap().sequence_number, 0);
        assert_eq!(buffer.get_next().unwrap().sequence_number, 1);
    }

    #[test]
    fn test_jitter_buffer_duplicate_handling() {
        let mut buffer = JitterBuffer::new(10);

        let packet1 = RTPPacket {
            payload: vec![1, 2, 3],
            sequence_number: 5,
            timestamp: 1000,
            ssrc: 12345,
        };

        buffer.insert(packet1).unwrap();

        // Insert duplicate with different payload
        let packet1_dup = RTPPacket {
            payload: vec![9, 9, 9],
            sequence_number: 5,
            timestamp: 1000,
            ssrc: 12345,
        };

        buffer.insert(packet1_dup).unwrap();

        // Should keep original
        assert_eq!(buffer.len(), 1);
        let retrieved = buffer.get_next().unwrap();
        assert_eq!(retrieved.payload, vec![1, 2, 3]);
    }

    #[test]
    fn test_jitter_buffer_capacity() {
        let mut buffer = JitterBuffer::new(3);

        // Fill buffer
        for i in 0..3 {
            buffer.insert(RTPPacket {
                payload: vec![i],
                sequence_number: i as u16,
                timestamp: 1000,
                ssrc: 12345,
            }).unwrap();
        }

        // Fourth insert should fail
        let result = buffer.insert(RTPPacket {
            payload: vec![3],
            sequence_number: 3,
            timestamp: 1000,
            ssrc: 12345,
        });

        assert!(result.is_err());
    }
}
