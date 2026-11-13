# Product Requirements Document

## 1. Product vision

AetherWall is a Linux-native wallpaper platform that lets users discover, organize, customize, create, and run static, animated, web-based, 3D, and interactive desktop wallpapers.

## 2. Target users

- KDE Plasma users who want more than static backgrounds.
- Users who maintain local wallpaper collections.
- Creators building HTML/CSS/JS wallpapers.
- Users who want animated, 3D, or small interactive desktop experiences.
- Anime wallpaper enthusiasts who want searchable online catalogs.

## 3. Product surfaces

### Desktop preview

A live preview area at the top of the Studio window. It represents the selected monitor and current wallpaper output. It is a preview surface, not a system-monitoring overlay.

### Studio

The main application below the preview. It contains:
- Home/overview
- Library
- Workshop/discovery
- Eight category pages
- Wallpaper details and customization
- Favorites and downloads
- Displays
- Settings
- Diagnostics/log viewer

## 4. Functional requirements

### Shared library
- Import local files and folders.
- Index supported media and extract thumbnails/metadata.
- Search, sort, filter, tag, favorite, remove and re-scan.
- Distinguish installed/local content from online discovery results.
- Preserve source attribution and license/usage metadata where available.

### Pictures
- Display supported image formats.
- Support fit, fill, crop, center and stretch.
- Provide per-display assignment.
- Preview before applying.

### Slideshow
- Build playlists from files, folders or collections.
- Sequential, shuffled and random modes.
- Configurable interval and transitions.
- Pause/resume and per-display assignment.

### Videos
- Local video playback using libmpv.
- Muted by default.
- Loop, playback speed, trim/seek preview and hardware-decoding selection where available.
- Expose actual codec, resolution, duration and frame-rate metadata when known.

### Web wallpapers
- HTML, CSS, JavaScript, Canvas and supported WebGL/WebGPU capabilities.
- Package-based import and validation.
- Live preview and developer console.
- Permission and resource controls.
- No unrestricted filesystem, shell, or network access by default.

### Anime pictures and videos
- Provider adapters with independent enable/disable controls.
- Search, tags, category, resolution and media-type filters where provider supports them.
- Download/cache and source attribution.
- Respect provider terms, rate limits, copyright and content restrictions.
- Provider failure must not break the rest of the application.

### 3D wallpapers
- glTF/GLB import.
- Scene preview, camera, lighting, environment, animation and quality settings.
- GPU capability detection and fallback.
- Resource limits and automatic suspension when not visible.

### Wallpaper games
- Small interactive experiences, initially based on a constrained HTML5 runtime.
- Input mapping and focus policy.
- Pause/resume and save state.
- CPU/GPU/memory limits.
- No arbitrary executable package payloads.

## 5. Non-functional requirements

- UI remains responsive during import, indexing, download and playback changes.
- Wallpaper runtime crashes do not crash Studio or daemon.
- Runtime state can be recovered after a runtime restart.
- No telemetry is collected or transmitted by default.
- Clear user-visible permissions for network and local file access.
- Performance presets and configurable FPS limits.
- Accessibility: keyboard navigation, readable contrast, scalable UI.
- Multi-monitor behaviour must be explicit and testable.
- Unsupported compositor features must be reported accurately.

## 6. Out of scope for the first product baseline

- System-monitoring HUD on the desktop.
- Arbitrary native executable plugins from wallpaper packages.
- Built-in social network or account system.
- Cloud sync as a mandatory dependency.
- Guaranteed support for every Wayland compositor at launch.
- DRM circumvention or scraping that violates provider terms.

## 7. Success measures

- A user can import, preview, apply, replace and remove a local image.
- A user can create and run a slideshow.
- A user can run a muted local video wallpaper.
- Studio remains usable if a wallpaper runtime crashes.
- Per-display assignments are correctly restored after restart.
- A web wallpaper is permission-gated and resource-limited.
- Package validation rejects malformed or unsafe manifests.
