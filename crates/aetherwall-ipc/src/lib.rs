//! # AetherWall IPC
//!
//! D-Bus interface between the AetherWall Studio and the daemon.
//!
//! Service name: `org.aetherwall.daemon.1`
//! Object path: `/org/aetherwall/daemon/1`

pub mod dbus_interface;
pub mod error;
pub mod service;

pub use dbus_interface::*;
pub use error::*;
