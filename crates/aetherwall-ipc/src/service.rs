//! # D-Bus service
//!
//! D-Bus service implementation for the daemon. Provides the signal and
//! method dispatch logic for the `org.aetherwall.daemon.1` interface.
pub use crate::dbus_interface::*;

use serde::{Deserialize, Serialize};

/// D-Bus method calls supported by the daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MethodCall {
    GetApplicationStatus(DbusRequest),
    GetAvailableDisplays(DbusRequest),
    GetWallpaperLibrary(DbusRequest),
    ImportWallpaper(DbusRequest),
    RemoveWallpaper(DbusRequest),
    SetActiveWallpaper(DbusRequest),
    StopActiveWallpaper(DbusRequest),
    GetCurrentWallpaper(DbusRequest),
    UpdateApplicationSettings(DbusRequest),
    GetRuntimeErrors(DbusRequest),
}

/// D-Bus signal emissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Signal {
    LibraryItemAdded { id: String },
    LibraryItemUpdated { id: String },
    LibraryItemRemoved { id: String },
    ImportProgress {
        path: String,
        bytes_received: u64,
        bytes_total: Option<u64>,
    },
    SessionStateChanged {
        session_id: String,
        state: String,
    },
    DisplayTopologyChanged,
    RuntimeHealthChanged {
        runtime_type: String,
        session_id: String,
        healthy: bool,
    },
    ErrorReported {
        error_code: String,
        error_message: String,
    },
}
