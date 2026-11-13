//! # AetherWall Core
//!
//! Shared domain types, traits, and error definitions used across all
//! AetherWall crates.
//!
//! This module defines the **runtime lifecycle contract** (ADR-003): every
//! wallpaper runtime adapter must implement the `WallpaperRuntime` trait.

pub mod display;
pub mod error;
pub mod models;
pub mod runtime;
pub mod session;
pub mod state;

pub use display::*;
pub use error::*;
pub use models::*;
pub use runtime::*;
pub use session::*;
pub use state::*;
