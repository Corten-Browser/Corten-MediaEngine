//! RTP packet handling
//!
//! Implements RTP (Real-time Transport Protocol) packet structure and packetization.

use std::cell::Cell;

/// Maximum Transmission Unit for RTP packets (bytes)
const RTP_MTU: usize = 1200;

/// RTP packet structure
///
/// Represents an RTP packet with header fields and payload.
///
/// # Examples
///
/// ```
/// use cortenbrowser_webrtc_integration::RTPPacket;
///
/// let packet = RTPPacket {
///     payload: vec![1, 2, 3, 4, 5],
///     sequence_number: 100,
///     timestamp: 1000,
///     ssrc: 0x12345678,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RTPPacket {
    /// Packet payload data
    pub payload: Vec<u8>,
    /// Sequence number (increments for each packet)
    pub sequence_number: u16,
    /// Timestamp (from media clock)
    pub timestamp: u32,
    /// Synchronization source identifier
    pub ssrc: u32,
}

impl RTPPacket {
    /// Serialize RTP packet to bytes
    ///
    /// Creates a properly formatted RTP packet with:
    /// - Version 2
    /// - Fixed 12-byte header
    /// - Payload appended after header
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_webrtc_integration::RTPPacket;
    ///
    /// let packet = RTPPacket {
    ///     payload: vec![0xAA, 0xBB],
    ///     sequence_number: 42,
    ///     timestamp: 9000,
    ///     ssrc: 0xDEADBEEF,
    /// };
    ///
    /// let bytes = packet.to_bytes();
    /// assert!(bytes.len() >= 12 + 2);
    /// ```
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12 + self.payload.len());

        // Byte 0: Version (2 bits) = 2, P=0, X=0, CC=0
        // 10 00 0000 = 0x80
        bytes.push(0x80);

        // Byte 1: M=0, PT=0 (payload type, can be extended later)
        bytes.push(0x00);

        // Bytes 2-3: Sequence number (big-endian)
        bytes.extend_from_slice(&self.sequence_number.to_be_bytes());

        // Bytes 4-7: Timestamp (big-endian)
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());

        // Bytes 8-11: SSRC (big-endian)
        bytes.extend_from_slice(&self.ssrc.to_be_bytes());

        // Bytes 12+: Payload
        bytes.extend_from_slice(&self.payload);

        bytes
    }
}

/// RTP packetizer for fragmenting payloads
///
/// Handles fragmentation of large payloads into RTP packets
/// that fit within MTU constraints.
///
/// # Examples
///
/// ```
/// use cortenbrowser_webrtc_integration::RTPPacketizer;
///
/// let packetizer = RTPPacketizer::new();
/// let payload = vec![1, 2, 3, 4, 5];
/// let packets = packetizer.packetize(&payload, 1000);
/// assert_eq!(packets.len(), 1);
/// ```
pub struct RTPPacketizer {
    sequence_number: Cell<u16>,
    ssrc: u32,
}

impl RTPPacketizer {
    /// Create a new RTP packetizer
    ///
    /// Generates a random SSRC for this stream.
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let ssrc = rng.gen();

        Self {
            sequence_number: Cell::new(0),
            ssrc,
        }
    }

    /// Set the sequence number
    ///
    /// Useful for testing or resuming streams.
    ///
    /// # Arguments
    ///
    /// * `seq` - The sequence number to set
    pub fn set_sequence_number(&mut self, seq: u16) {
        self.sequence_number.set(seq);
    }

    /// Packetize a payload into RTP packets
    ///
    /// Fragments large payloads into multiple packets if needed.
    /// Each packet will have the same timestamp but incrementing
    /// sequence numbers.
    ///
    /// # Arguments
    ///
    /// * `payload` - The payload data to packetize
    /// * `timestamp` - The RTP timestamp for this payload
    ///
    /// # Returns
    ///
    /// A vector of RTP packets
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_webrtc_integration::RTPPacketizer;
    ///
    /// let packetizer = RTPPacketizer::new();
    /// let large_payload = vec![0u8; 3000];
    /// let packets = packetizer.packetize(&large_payload, 5000);
    /// assert!(packets.len() > 1); // Fragmented
    /// ```
    pub fn packetize(&self, payload: &[u8], timestamp: u32) -> Vec<RTPPacket> {
        if payload.is_empty() {
            return vec![];
        }

        let mut packets = Vec::new();
        let mut offset = 0;

        while offset < payload.len() {
            let chunk_size = std::cmp::min(RTP_MTU, payload.len() - offset);
            let chunk = payload[offset..offset + chunk_size].to_vec();

            let seq = self.sequence_number.get();
            let packet = RTPPacket {
                payload: chunk,
                sequence_number: seq,
                timestamp,
                ssrc: self.ssrc,
            };

            packets.push(packet);

            // Increment sequence number with wraparound
            self.sequence_number.set(seq.wrapping_add(1));
            offset += chunk_size;
        }

        packets
    }
}

impl Default for RTPPacketizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtp_packet_to_bytes() {
        let packet = RTPPacket {
            payload: vec![0xAA, 0xBB, 0xCC],
            sequence_number: 42,
            timestamp: 9000,
            ssrc: 0xDEADBEEF,
        };

        let bytes = packet.to_bytes();

        // Verify length
        assert_eq!(bytes.len(), 12 + 3);

        // Verify version and flags
        assert_eq!(bytes[0], 0x80); // Version 2

        // Verify sequence number
        assert_eq!(u16::from_be_bytes([bytes[2], bytes[3]]), 42);

        // Verify timestamp
        assert_eq!(u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]), 9000);

        // Verify SSRC
        assert_eq!(u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]), 0xDEADBEEF);

        // Verify payload
        assert_eq!(&bytes[12..], &[0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn test_packetizer_mtu_fragmentation() {
        let packetizer = RTPPacketizer::new();
        let large_payload = vec![0x42; 3000];

        let packets = packetizer.packetize(&large_payload, 5000);

        // Should be fragmented
        assert!(packets.len() > 1);

        // Verify total size matches
        let total_size: usize = packets.iter().map(|p| p.payload.len()).sum();
        assert_eq!(total_size, 3000);
    }

    #[test]
    fn test_packetizer_sequence_increment() {
        let packetizer = RTPPacketizer::new();

        let packets1 = packetizer.packetize(&vec![1, 2, 3], 1000);
        let packets2 = packetizer.packetize(&vec![4, 5, 6], 2000);

        assert_eq!(packets2[0].sequence_number, packets1[0].sequence_number + 1);
    }
}
