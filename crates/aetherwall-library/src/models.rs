
//! # Library models
//!
//! Data structures for the wallpaper library.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallpaper {
    pub id: String,
    pub wallpaper_type: String,
    pub source_kind: String,
    pub title: String,
    pub description: Option<String>,
    pub content_path: String,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_ms: Option<u64>,
    pub file_size: Option<u64>,
    pub mime_type: Option<String>,
    pub author: Option<String>,
    pub license_id: Option<String>,
    pub is_favorite: bool,
    pub display_id: Option<String>,
    pub last_used_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Display {
    pub id: String,
    pub backend_key: String,
    pub name: Option<String>,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub refresh_rate_mhz: Option<u32>,
    pub is_primary: bool,
    pub position_x: i32,
    pub position_y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub wallpaper_id: String,
    pub display_id: Option<String>,
    pub runtime_type: String,
    pub state: String,
    pub runtime_session_id: Option<String>,
    pub last_error_code: Option<String>,
    pub last_error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
