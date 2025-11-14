//! # buffer_manager Component
//!
//! Memory buffer and cache management (ring buffers, frame caches, memory tracking)
//!
//! This crate provides efficient memory buffers and caches for the Corten Media Engine:
//!
//! - [`RingBuffer`] - Circular buffer for streaming byte data
//! - [`FrameCache`] - LRU cache for video frames
//! - [`BufferManager`] - Coordinates buffer resources and memory limits
//!
//! # Examples
//!
//! Creating a ring buffer:
//!
//! ```
//! use cortenbrowser_buffer_manager::RingBuffer;
//!
//! let mut buffer = RingBuffer::new(1024);
//! buffer.write(b"Hello, world!").unwrap();
//!
//! let mut out = vec![0u8; 13];
//! let n = buffer.read(&mut out).unwrap();
//! assert_eq!(n, 13);
//! assert_eq!(&out, b"Hello, world!");
//! ```

#![warn(missing_docs)]

mod config;
mod error;
mod ring;
mod cache;
mod manager;

pub use config::BufferConfig;
pub use error::BufferError;
pub use ring::RingBuffer;
pub use cache::FrameCache;
pub use manager::{BufferManager, VideoFrameBuffer, AudioSampleBuffer};
