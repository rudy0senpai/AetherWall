//! # Slideshow runtime
//!
//! Slideshow wallpaper runtime with configurable interval and transitions.

use core::display::Display;
use core::models::{Wallpaper, WallpaperType, Window};
use core::runtime::{HealthStatus, WallpaperRuntime};
use core::session::SessionState as SessionState_;
use std::time::Duration;
use tokio::time;

#[derive(Debug, Clone)]
pub struct SlideshowRuntime {
    pub interval_ms: u64,
    pub shuffle: bool,
    pub transition: Option<String>,
}

impl SlideshowRuntime {
    pub fn new(interval_ms: u64, shuffle: bool, transition: Option<String>) -> Self {
        Self {
            interval_ms,
            shuffle,
            transition,
        }
    }

    /// Load image paths from a directory or playlist.
    pub fn load_images(&self, dir: &str) -> Result<Vec<String>, String> {
        let mut paths = Vec::new();
        if dir.is_empty() {
            return Ok(paths);
        }
        let entries = walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok());
        for entry in entries {
            let path = entry.path();
            if path.is_file() {
                if ImageRuntime::validate_extension(path.to_string_lossy().as_ref()).is_ok() {
                    paths.push(path.to_string_lossy().to_string());
                }
            }
        }
        Ok(paths)
    }
}

#[async_trait::async_trait]
impl WallpaperRuntime for SlideshowRuntime {
    fn runtime_type(&self) -> &'static str {
        "slideshow"
    }

    fn supported_types(&self) -> &'static [WallpaperType] {
        &[WallpaperType::Slideshow]
    }

    async fn validate(
        &self,
        wallpaper: &Wallpaper,
    ) -> Result<(), Box<core::error::AetherWallError>> {
        if wallpaper.wallpaper_type != WallpaperType::Slideshow {
            return Err(Box::new(core::error::AetherWallError::InvalidInput(
                "slideshow runtime cannot handle this type".to_string(),
            )));
        }
        Ok(())
    }

    async fn prepare(
        &self,
        wallpaper: &Wallpaper,
        _display: &Display,
        settings: &Window,
    ) -> Result<(), Box<core::error::AetherWallError>> {
        if let Some(ref interval) = settings.interval_ms {
            self.interval_ms = *interval;
        }
        if settings.shuffle == Some(true) {
            self.shuffle = true;
        }
        Ok(())
    }

    async fn start(&self, session_id: &str) -> Result<(), Box<core::error::AetherWallError>> {
        // Slideshow: pre-load playlist and begin iteration
        time::sleep(time::Duration::from_millis(self.interval_ms / 2)).await;
        Ok(())
    }

    async fn pause(&self, _session_id: &str, reason: &str) -> Result<(), Box<core::error::AetherWallError>> {
        // In a real implementation, pause the slideshow timer
        Ok(())
    }

    async fn resume(&self, _session_id: &str) -> Result<(), Box<core::error::AetherWallError>> {
        Ok(())
    }

    async fn update_settings(
        &self,
        _session_id: &str,
        settings: &Window,
    ) -> Result<(), Box<core::error::AetherWallError>> {
        if let Some(ref interval) = settings.interval_ms {
            self.interval_ms = *interval;
        }
        if settings.shuffle == Some(true) {
            self.shuffle = true;
        }
        Ok(())
    }

    async fn stop(&self, _session_id: &str) -> Result<(), Box<core::error::AetherWallError>> {
        Ok(())
    }

    async fn health(
        &self,
        _session_id: &str,
    ) -> Result<HealthStatus, Box<core::error::AetherWallError>> {
        Ok(HealthStatus::Healthy)
    }

    async fn dispose(&self, _session_id: &str) -> Result<(), Box<core::error::AetherWallError>> {
        Ok(())
    }
}
