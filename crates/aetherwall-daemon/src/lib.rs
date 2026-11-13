//! # AetherWall Daemon
//!
//! Background service that owns wallpaper catalog, sessions, scheduling,
//! display mapping, settings, runtime supervision, and event delivery.
//!
//! The daemon persists state through SQLite and exposes a versioned D-Bus
//! contract (`org.aetherwall.daemon.1`). The Studio UI communicates with the
//! daemon over D-Bus and does not directly manage long-running playback.

pub mod config;
pub mod error;
pub mod event_bus;
pub mod runtime_registry;
pub mod session_manager;
pub mod storage;

pub use config::*;
pub use error::*;
pub use event_bus::*;
pub use runtime_registry::*;
pub use session_manager::*;
pub use storage::*;
