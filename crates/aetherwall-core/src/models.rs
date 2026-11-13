//! # Domain models
//!
//! Core data structures shared across the AetherWall ecosystem.

use serde::{Deserialize, Serialize};

/// Media type of a wallpaper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WallpaperType {
    /// Static image.
    Image,
    /// Slideshow of images.
    Slideshow,
    /// Video (libmpv).
    Video,
    /// HTML/CSS/JS web wallpaper.
    Web,
    /// Anime image from a provider.
    AnimeImage,
    /// Anime video from a provider.
    AnimeVideo,
    /// 3D scene (glTF/GLB).
    Scene3D,
    /// Interactive game.
    Game,
}

impl std::fmt::Display for WallpaperType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Image => write!(f, "image"),
            Self::Slideshow => write!(f, "slideshow"),
            Self::Video => write!(f, "video"),
            Self::Web => write!(f, "web"),
            Self::AnimeImage => write!(f, "anime-image"),
            Self::AnimeVideo => write!(f, "anime-video"),
            Self::Scene3D => write!(f, "3d"),
            Self::Game => write!(f, "game"),
        }
    }
}

/// Source of a wallpaper (local, provider, bundled).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceKind {
    Local,
    Provider,
    Bundled,
}

/// Availability state of a wallpaper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailabilityState {
    Available,
    Downloading,
    Imported,
    Broken,
    Offline,
}

/// Metadata extracted from an imported image file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    /// File extension.
    pub path: String,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// File size in bytes.
    pub file_size: u64,
    /// Detected MIME type.
    pub mime_type: Option<String>,
    /// Duration (for animated formats) in milliseconds.
    pub duration_ms: Option<u64>,
}

/// A wallpaper entry in the library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallpaper {
    /// UUID.
    pub id: String,
    /// Runtime type (e.g. "image", "slideshow", "video").
    pub wallpaper_type: WallpaperType,
    /// Source kind.
    pub source_kind: SourceKind,
    /// Title.
    pub title: String,
    /// Description.
    pub description: Option<String>,
    /// Path to the primary content file.
    pub content_path: String,
    /// Path to the generated thumbnail.
    pub thumbnail_path: Option<String>,
    /// Preview path.
    pub preview_path: Option<String>,
    /// Width in pixels.
    pub width: Option<u32>,
    /// Height in pixels.
    pub height: Option<u32>,
    /// Duration in milliseconds.
    pub duration_ms: Option<u64>,
    /// File size in bytes.
    pub file_size: Option<u64>,
    /// MIME type.
    pub mime_type: Option<String>,
    /// Author/source attribution.
    pub author: Option<String>,
    /// License identifier.
    pub license_id: Option<String>,
    /// Whether this wallpaper is a favorite.
    pub is_favorite: bool,
    /// Reference to the display this wallpaper is applied to (if any).
    pub display_id: Option<String>,
    /// Last used timestamp.
    pub last_used_at: Option<String>,
}

/// A display assignment: which wallpaper runs on which display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayAssignment {
    /// Display the wallpaper is applied to.
    pub display_id: String,
    /// Wallpaper to apply.
    pub wallpaper_id: String,
    /// Type of wallpaper being applied.
    pub wallpaper_type: WallpaperType,
    /// Fit/fill/crop/center/stretch mode.
    pub fit_mode: FitMode,
}

/// Display wallpaper fit mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FitMode {
    /// Stretch to fill (may distort).
    Stretch,
    /// Fill the display (lettersboxed).
    Fill,
    /// Fit contained within display (letterboxed).
    Fit,
    /// Center only.
    Center,
    /// Cover (crop to fill).
    Cover,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallpaper_type_display() {
        assert_eq!(WallpaperType::Image.to_string(), "image");
        assert_eq!(WallpaperType::Slideshow.to_string(), "slideshow");
        assert_eq!(WallpaperType::Video.to_string(), "video");
        assert_eq!(WallpaperType::Web.to_string(), "web");
        assert_eq!(WallpaperType::AnimeImage.to_string(), "anime-image");
        assert_eq!(WallpaperType::AnimeVideo.to_string(), "anime-video");
        assert_eq!(WallpaperType::Scene3D.to_string(), "3d");
        assert_eq!(WallpaperType::Game.to_string(), "game");
    }
}
