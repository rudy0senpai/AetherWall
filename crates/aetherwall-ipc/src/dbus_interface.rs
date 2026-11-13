//! # D-Bus interface definitions
//!
//! Strongly-typed D-Bus message structures for Studio-daemon communication.
//!
//! Interface: `org.aetherwall.daemon.1`

use serde::{Deserialize, Serialize};

/// Version of the IPC contract.
pub const CONTRACT_VERSION: &str = "1.0";

/// Request sent from Studio to Daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbusRequest {
    /// Max version accepted (client is compatible with <= this).
    pub client_max_version: String,
    /// Method name.
    pub method: String,
    /// Serial number for this call.
    pub serial: u64,
    /// Request payload (typed per method).
    pub payload: serde_json::Value,
}

/// Response sent from Daemon to Studio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbusResponse {
    /// Serial number matching the request.
    pub serial: u64,
    /// Whether the call succeeded.
    pub success: bool,
    /// Result payload (typed per method).
    pub payload: serde_json::Value,
    /// Error name if success=false.
    pub error_name: Option<String>,
    /// Error message if success=false.
    pub error_message: Option<String>,
}

/// Async status of a D-Bus call.
pub enum CallStatus {
    Pending,
    Completed(DbusResponse),
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub pid: Option<u32>,
    pub state: String,
    pub supported_runtimes: Vec<String>,
    pub database_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaysResponse {
    pub displays: Vec<DisplayInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayInfo {
    pub id: String,
    pub name: Option<String>,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryResponse {
    pub wallpapers: Vec<LibraryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
    pub id: String,
    pub wallpaper_type: String,
    pub source_kind: String,
    pub title: String,
    pub content_path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub file_size: Option<u64>,
    pub is_favorite: bool,
    pub display_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResponse {
    pub success: bool,
    pub wallpaper_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub settings: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub error_message: String,
}
