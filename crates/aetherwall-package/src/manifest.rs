//! # Package manifest
//!
//! Schema for the `.awall` package manifest.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub id: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeInfo {
    pub id: String,
    pub minimum_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub network: bool,
    pub audio: bool,
    pub mouse_input: bool,
    pub keyboard_input: bool,
    pub filesystem_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub max_fps: u32,
    pub memory_limit_mb: u32,
    pub cpu_budget_percent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperManifest {
    #[serde(default)]
    pub schema_version: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub r#type: String,
    pub entrypoint: Option<String>,
    pub preview: Option<String>,
    pub thumbnail: Option<String>,
    pub author: Option<Author>,
    pub license: Option<LicenseInfo>,
    pub runtime: Option<RuntimeInfo>,
    #[serde(default)]
    pub capabilities: Capabilities,
    #[serde(default)]
    pub resources: Resources,
    #[serde(default)]
    pub tags: Vec<String>,
}
