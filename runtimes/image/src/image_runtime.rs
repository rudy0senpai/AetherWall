//! # Image runtime
//!
//! Image wallpaper runtime implementing the `WallpaperRuntime` trait.

use std::path::Path;

use core::display::Display;
use core::models::{Wallpaper, WallpaperType, Window};
use core::runtime::{HealthStatus, WallpaperRuntime};
use core::session::SessionState;

#[derive(Debug, Clone)]
pub struct ImageRuntime {
    /// Thumbnail cache directory.
    pub thumbnail_cache: String,
}

impl ImageRuntime {
    pub fn new(thumbnail_cache: &str) -> Self {
        Self {
            thumbnail_cache: thumbnail_cache.to_string(),
        }
    }

    /// Validate an image file by extension.
    pub fn validate_extension(path: &str) -> Result<(), String> {
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .ok_or("no file extension")?;
        let allowed = ["png", "jpg", "jpeg", "webp", "bmp"];
        if allowed.contains(&ext.to_lowercase().as_str()) {
            Ok(())
        } else {
            Err(format!("unsupported image format: {{}}", ext))
        }
    }

    /// Generate a thumbnail path for an image.
    pub fn thumbnail_path(&self, image_path: &str) -> String {
        let name = Path::new(image_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("thumb");
        format!("{{}}@thumb.png", name)
    }
}

#[async_trait::async_trait]
impl WallpaperRuntime for ImageRuntime {
    fn runtime_type(&self) -> &'static str {
        "image"
    }

    fn supported_types(&self) -> &'static [WallpaperType] {
        &[WallpaperType::Image]
    }

    async fn validate(
        &self,
        wallpaper: &Wallpaper,
    ) -> Result<(), Box<core::error::AetherWallError>> {
        if wallpaper.wallpaper_type != WallpaperType::Image {
            return Err(Box::new(core::error::AetherWallError::InvalidInput(
                "image runtime cannot handle this type".to_string(),
            )));
        }
        if !Path::new(&wallpaper.content_path).exists() {
            return Err(Box::new(core::error::AetherWallError::IoError(
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("image file not found: {{}}", wallpaper.content_path),
                ),
            )));
        }
        ImageRuntime::validate_extension(&wallpaper.content_path)?;
        Ok(())
    }

    async fn prepare(
        &self,
        wallpaper: &Wallpaper,
        display: &Display,
        settings: &Window,
    ) -> Result<(), Box<core::error::AetherWallError>> {
        if !Path::new(&wallpaper.content_path).exists() {
            return Err(Box::new(core::error::AetherWallError::IoError(
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("image file not found: {{}}", wallpaper.content_path),
                ),
            )));
        }
        if settings.display_id != display.id {
            return Err(Box::new(core::error::AetherWallError::UnknownDisplay(
                settings.display_id.clone(),
            )));
        }
        Ok(())
    }

    async fn start(&self, session_id: &str) -> Result<(), Box<core::error::AetherWallError>> {
        // Image rendering is near-instant; simulate background preload
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        // In a real implementation, launch the native image renderer here
        Ok(())
    }

    async fn pause(
        &self,
        _session_id: &str,
        _reason: &str,
    ) -> Result<(), Box<core::error::AetherWallError>> {
        Ok(())
    }

    async fn resume(&self, _session_id: &str) -> Result<(), Box<core::error::AetherWallError>> {
        Ok(())
    }

    async fn update_settings(
        &self,
        _session_id: &str,
        _settings: &Window,
    ) -> Result<(), Box<core::error::AetherWallError>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_validation() {
        assert!(ImageRuntime::validate_extension("photo.png").is_ok());
        assert!(ImageRuntime::validate_extension("photo.jpg").is_ok());
        assert!(ImageRuntime::validate_extension("photo.jpeg").is_ok());
        assert!(ImageRuntime::validate_extension("photo.webp").is_ok());
        assert!(ImageRuntime::validate_extension("photo.bmp").is_ok());
        assert!(ImageRuntime::validate_extension("photo.mp4").is_err());
        assert!(ImageRuntime::validate_extension("photo").is_err());
    }

    #[test]
    fn test_thumbnail_path() {
        let rt = ImageRuntime::new("cache");
        assert_eq!(
            rt.thumbnail_path("/home/user/Pictures/photo.png"),
            "photo@thumb.png"
        );
    }
}
