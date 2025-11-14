//! Ring buffer implementation for streaming data
//!
//! A circular buffer that efficiently manages byte streams with wraparound.

use crate::error::BufferError;

/// A circular buffer for streaming byte data
///
/// The RingBuffer provides an efficient FIFO buffer that wraps around when
/// reaching the end, avoiding the need to move data in memory.
///
/// # Examples
///
/// ```
/// use cortenbrowser_buffer_manager::RingBuffer;
///
/// let mut buffer = RingBuffer::new(1024);
///
/// // Write data
/// let written = buffer.write(b"Hello").unwrap();
/// assert_eq!(written, 5);
///
/// // Read data
/// let mut out = vec![0u8; 5];
/// let read = buffer.read(&mut out).unwrap();
/// assert_eq!(read, 5);
/// assert_eq!(&out, b"Hello");
/// ```
#[derive(Debug)]
pub struct RingBuffer {
    buffer: Vec<u8>,
    capacity: usize,
    read_pos: usize,
    write_pos: usize,
    count: usize,
}

impl RingBuffer {
    /// Creates a new ring buffer with the specified capacity
    ///
    /// # Arguments
    ///
    /// * `capacity` - The maximum number of bytes the buffer can hold
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::RingBuffer;
    ///
    /// let buffer = RingBuffer::new(1024);
    /// assert_eq!(buffer.capacity(), 1024);
    /// assert_eq!(buffer.available(), 0);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0; capacity],
            capacity,
            read_pos: 0,
            write_pos: 0,
            count: 0,
        }
    }

    /// Writes data to the ring buffer
    ///
    /// Returns the number of bytes written. If the buffer is full, returns
    /// `BufferError::BufferFull`.
    ///
    /// # Arguments
    ///
    /// * `data` - The bytes to write
    ///
    /// # Errors
    ///
    /// Returns `BufferError::BufferFull` if there is not enough space
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::RingBuffer;
    ///
    /// let mut buffer = RingBuffer::new(10);
    /// let written = buffer.write(b"Hello").unwrap();
    /// assert_eq!(written, 5);
    /// assert_eq!(buffer.available(), 5);
    /// ```
    pub fn write(&mut self, data: &[u8]) -> Result<usize, BufferError> {
        if data.is_empty() {
            return Ok(0);
        }

        let available_space = self.capacity - self.count;
        if available_space == 0 {
            return Err(BufferError::BufferFull);
        }

        let to_write = data.len().min(available_space);

        for &byte in data.iter().take(to_write) {
            self.buffer[self.write_pos] = byte;
            self.write_pos = (self.write_pos + 1) % self.capacity;
        }

        self.count += to_write;
        Ok(to_write)
    }

    /// Reads data from the ring buffer
    ///
    /// Returns the number of bytes read. If the buffer is empty, returns
    /// `BufferError::BufferEmpty`.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer to read into
    ///
    /// # Errors
    ///
    /// Returns `BufferError::BufferEmpty` if no data is available
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::RingBuffer;
    ///
    /// let mut buffer = RingBuffer::new(10);
    /// buffer.write(b"Hello").unwrap();
    ///
    /// let mut out = vec![0u8; 5];
    /// let read = buffer.read(&mut out).unwrap();
    /// assert_eq!(read, 5);
    /// assert_eq!(&out, b"Hello");
    /// ```
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, BufferError> {
        if self.count == 0 {
            return Err(BufferError::BufferEmpty);
        }

        let to_read = buf.len().min(self.count);

        for byte_ref in buf.iter_mut().take(to_read) {
            *byte_ref = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % self.capacity;
        }

        self.count -= to_read;
        Ok(to_read)
    }

    /// Returns the number of bytes available to read
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::RingBuffer;
    ///
    /// let mut buffer = RingBuffer::new(100);
    /// assert_eq!(buffer.available(), 0);
    ///
    /// buffer.write(b"Test").unwrap();
    /// assert_eq!(buffer.available(), 4);
    /// ```
    pub fn available(&self) -> usize {
        self.count
    }

    /// Returns the total capacity of the buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use cortenbrowser_buffer_manager::RingBuffer;
    ///
    /// let buffer = RingBuffer::new(1024);
    /// assert_eq!(buffer.capacity(), 1024);
    /// ```
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer_is_empty() {
        let buffer = RingBuffer::new(100);
        assert_eq!(buffer.capacity(), 100);
        assert_eq!(buffer.available(), 0);
    }

    #[test]
    fn test_write_and_read_simple() {
        let mut buffer = RingBuffer::new(100);

        // Write data
        let written = buffer.write(b"Hello, world!").unwrap();
        assert_eq!(written, 13);
        assert_eq!(buffer.available(), 13);

        // Read data
        let mut out = vec![0u8; 13];
        let read = buffer.read(&mut out).unwrap();
        assert_eq!(read, 13);
        assert_eq!(&out, b"Hello, world!");
        assert_eq!(buffer.available(), 0);
    }

    #[test]
    fn test_read_from_empty_buffer() {
        let mut buffer = RingBuffer::new(100);
        let mut out = vec![0u8; 10];

        let result = buffer.read(&mut out);
        assert_eq!(result, Err(BufferError::BufferEmpty));
    }

    #[test]
    fn test_write_to_full_buffer() {
        let mut buffer = RingBuffer::new(10);

        // Fill the buffer
        buffer.write(b"0123456789").unwrap();

        // Try to write more
        let result = buffer.write(b"X");
        assert_eq!(result, Err(BufferError::BufferFull));
    }

    #[test]
    fn test_wraparound() {
        let mut buffer = RingBuffer::new(10);

        // Write and read to move positions
        buffer.write(b"12345").unwrap();
        let mut tmp = vec![0u8; 5];
        buffer.read(&mut tmp).unwrap();

        // Write more data (should wrap around)
        buffer.write(b"ABCDEFGH").unwrap();
        assert_eq!(buffer.available(), 8);

        // Read it back
        let mut out = vec![0u8; 8];
        buffer.read(&mut out).unwrap();
        assert_eq!(&out, b"ABCDEFGH");
    }

    #[test]
    fn test_partial_read() {
        let mut buffer = RingBuffer::new(100);
        buffer.write(b"Hello, world!").unwrap();

        // Read only part of the data
        let mut out = vec![0u8; 5];
        let read = buffer.read(&mut out).unwrap();
        assert_eq!(read, 5);
        assert_eq!(&out, b"Hello");
        assert_eq!(buffer.available(), 8);

        // Read the rest
        let mut out2 = vec![0u8; 8];
        let read2 = buffer.read(&mut out2).unwrap();
        assert_eq!(read2, 8);
        assert_eq!(&out2, b", world!");
    }

    #[test]
    fn test_multiple_write_read_cycles() {
        let mut buffer = RingBuffer::new(50);

        for i in 0..10 {
            let data = format!("Message {}", i);
            buffer.write(data.as_bytes()).unwrap();

            let mut out = vec![0u8; data.len()];
            buffer.read(&mut out).unwrap();
            assert_eq!(out, data.as_bytes());
        }

        assert_eq!(buffer.available(), 0);
    }
}
