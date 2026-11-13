//! # Settings
//!
//! Persisted application settings.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettingKey {
    Theme,
    Language,
    DefaultFitMode,
    ShowDesktopPreview,
    EnableDiagnostics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsStore {
    pub theme: String,
    pub language: String,
    pub default_fit_mode: String,
    pub show_desktop_preview: bool,
    pub enable_diagnostics: bool,
}

impl Default for SettingsStore {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            language: "en-US".to_string(),
            default_fit_mode: "fit".to_string(),
            show_desktop_preview: true,
            enable_diagnostics: false,
        }
    }
}
