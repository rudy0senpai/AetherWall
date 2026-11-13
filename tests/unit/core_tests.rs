// Unit tests for aetherwall-core
// Run: cargo test -p aetherwall-core --test core_tests

use aetherwall_core::*;

#[test]
fn test_fit_mode_str() {
    assert_eq!(FitMode::Stretch.to_string(), "stretch");
    assert_eq!(FitMode::Fill.to_string(), "fill");
    assert_eq!(FitMode::Fit.to_string(), "fit");
    assert_eq!(FitMode::Center.to_string(), "center");
    assert_eq!(FitMode::Cover.to_string(), "cover");
}

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

#[test]
fn test_display_pixel_resolution() {
    let d = Display::test("d1", 1920, 1080);
    assert_eq!(d.pixels(), (1920, 1080));
    let d2 = Display { scale_factor: 1.5, ..d };
    assert_eq!(d2.pixels(), (2880, 1620));
}

#[test]
fn test_session_transitions() {
    assert!(validate_transition(SessionState::Created, SessionState::Preparing).is_ok());
    assert!(validate_transition(SessionState::Prepared, SessionState::Running).is_ok());
    assert!(validate_transition(SessionState::Created, SessionState::Running).is_err());
}

#[test]
fn test_session_functions() {
    let mut s = Session::new(
        Wallpaper {
            id: "test-uuid".to_string(),
            wallpaper_type: WallpaperType::Image,
            source_kind: SourceKind::Local,
            title: "Test".to_string(),
            description: None,
            content_path: "/tmp/test.png".to_string(),
            thumbnail_path: None,
            preview_path: None,
            width: None,
            height: None,
            duration_ms: None,
            file_size: None,
            mime_type: None,
            author: None,
            license_id: None,
            is_favorite: false,
            display_id: None,
            last_used_at: None,
        },
        Display::test("d1", 1920, 1080),
        "image",
    );
    s.running();
    assert_eq!(s.state, SessionState::Running);
    s.paused("user request");
    assert_eq!(s.state, SessionState::Paused);
    s.failed("runtime-crash", "mpv exited");
    assert_eq!(s.state, SessionState::Failed);
}
