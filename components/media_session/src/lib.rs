//! # media_session Component
//!
//! Media session lifecycle and state management (session creation, state transitions, cleanup)

#![warn(missing_docs)]

mod manager;
mod session;
mod state;

pub use manager::SessionManager;
pub use session::MediaSession;
pub use state::{MediaMetadata, SessionState};
