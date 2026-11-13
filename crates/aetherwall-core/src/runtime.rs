//! # Runtime lifecycle contract (ADR-003)
//!
//! Every wallpaper runtime adapter must implement this trait. The contract is
//! versioned and stable; new operations are added only via new trait
//! implementations, never by modifying existing ones.

use super::{Display, FitMode, Wallpaper, WallpaperType, Window};
use async_trait::async_trait;
use std::time::Duration;

/// A runtime that renders a wallpaper.
#[async_trait]
pub trait WallpaperRuntime: Send + Sync {
    /// Unique identifier for this runtime type.
    fn runtime_type(&self) -> &'static str;

    /// Wallpaper types this runtime can handle.
    fn supported_types(&self) -> &'static [WallpaperType];

    /// Validate the wallpaper content without starting playback.
    async fn validate(&self, wallpaper: &Wallpaper) -> Result<(), Box<AetherWallError>>;

    /// Prepare the wallpaper for a specific display.
    async fn prepare(
        &self,
        wallpaper: &Wallpaper,
        display: &Display,
        settings: &Window,
    ) -> Result<(), Box<AetherWallError>>;

    /// Start the runtime.
    async fn start(&self, session_id: &str) -> Result<(), Box<AetherWallError>>;

    /// Pause the runtime.
    async fn pause(&self, session_id: &str, reason: &str) -> Result<(), Box<AetherWallError>>;

    /// Resume the runtime.
    async fn resume(&self, session_id: &str) -> Result<(), Box<AetherWallError>>;

    /// Update window settings for a running session.
    async fn update_settings(
        &self,
        session_id: &str,
        settings: &Window,
    ) -> Result<(), Box<AetherWallError>>;

    /// Stop the runtime.
    async fn stop(&self, session_id: &str) -> Result<(), Box<AetherWallError>>;

    /// Check runtime health.
    async fn health(&self, session_id: &str) -> Result<HealthStatus, Box<AetherWallError>>;

    /// Dispose resources (call after stop).
    async fn dispose(&self, session_id: &str) -> Result<(), Box<AetherWallError>>;
}

/// Status of a running runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Runtime is healthy.
    Healthy,
    /// Runtime is degraded (e.g., fallback shaders enabled).
    Degraded,
    /// Runtime has crashed or is not running.
    Failed,
    /// Runtime is stopped (not running).
    Stopped,
}

/// Current state of a wallpaper session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]


/// Window configuration for a wallpaper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    /// Display target (backend + id).
    pub display_id: String,
    /// Fit/fill/crop/center/stretch mode.
    pub fit_mode: FitMode,
    /// Wallpaper image path (for image runtimes).
    pub image_path: Option<String>,
    /// Slideshow interval in milliseconds.
    pub interval_ms: Option<u64>,
    /// Slideshow shuffle.
    pub shuffle: Option<bool>,
    /// Transition effect type.
    pub transition: Option<String>,
    /// Web wallpaper entrypoint.
    pub web_entrypoint: Option<String>,
    /// Custom settings for the specific runtime.
    pub custom: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_mode_str() {
        assert_eq!(FitMode::Stretch.to_string(), "stretch");
        assert_eq!(FitMode::Fill.to_string(), "fill");
        assert_eq!(FitMode::Fit.to_string(), "fit");
        assert_eq!(FitMode::Center.to_string(), "center");
        assert_eq!(FitMode::Cover.to_string(), "cover");
    }
}
