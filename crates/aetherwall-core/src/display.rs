//! # Display types
//!
//! Representation of physical and virtual display targets for wallpapers.

use serde::{Deserialize, Serialize};

/// A monitor/display detected on the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Display {
    /// Stable internal identifier (UUID).
    pub id: String,
    /// Backend display key (e.g. `DPI`, `DPM`, `DP-1`, `eDP-1` for X11).
    pub backend_key: String,
    /// Human-readable name (may be unknown).
    pub name: Option<String>,
    /// Physical width in millimeters.
    pub width_mm: Option<u32>,
    /// Physical height in millimeters.
    pub height_mm: Option<u32>,
    /// Logical width in pixels.
    pub logical_width: u32,
    /// Logical height in pixels.
    pub logical_height: u32,
    /// Scale factor relative to 96 DPI (e.g. 1.0, 1.25, 1.5, 2.0).
    pub scale_factor: f32,
    /// Refresh rate in millihertz (e.g. 60000 = 60 Hz).
    pub refresh_rate_mhz: Option<u32>,
    /// Whether this is the primary display.
    pub is_primary: bool,
    /// Position in the virtual desktop (x, y).
    pub position_x: i32,
    pub position_y: i32,
}

/// A windowing backend target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Backend {
    /// KDE Plasma 6 Wayland.
    Wayland,
    /// KDE Plasma 6 X11.
    X11,
}

impl Display {
    /// Resolve the logical pixel size in physical device pixels.
    pub fn pixels(self) -> (u32, u32) {
        (
            (self.logical_width as f32 * self.scale_factor).round() as u32,
            (self.logical_height as f32 * self.scale_factor).round() as u32,
        )
    }

    /// Create a synthetic display for testing.
    pub fn test(id: &str, width: u32, height: u32) -> Self {
        Display {
            id: id.to_string(),
            backend_key: "virtual".to_string(),
            name: None,
            width_mm: None,
            height_mm: None,
            logical_width: width,
            logical_height: height,
            scale_factor: 1.0,
            refresh_rate_mhz: Some(60000),
            is_primary: false,
            position_x: 0,
            position_y: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_resolution() {
        let d = Display::test("d1", 1920, 1080);
        assert_eq!(d.pixels(), (1920, 1080));
        let d2 = Display { scale_factor: 1.5, ..d };
        assert_eq!(d2.pixels(), (2880, 1620));
    }
}
