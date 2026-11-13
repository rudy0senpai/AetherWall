# UI/UX Specification

## 1. Design source of truth

The user's supplied reference images are the primary visual references. The interface combines:
- A desktop wallpaper preview at the top.
- A wallpaper-management application below.
- A dark, black/graphite base with purple and magenta accents.
- Large visual thumbnails and category chips.
- A professional launcher/library layout.
- Dedicated customization dialogs.
- A split editor/preview workspace for web wallpapers.
- A specialized 3D viewport/editor and game launcher.

Do not replace this direction with a generic admin dashboard.

## 2. Product shell

### Top desktop preview
- Occupies the upper portion of the main window.
- Shows the currently applied wallpaper for the selected display.
- Includes display selector and preview sizing controls.
- Preview should be live when feasible; a clearly labelled fallback snapshot is acceptable during initial prototype work.
- Must not show CPU/RAM/battery gauges, system graphs, telemetry cards, or the old reactive HUD.

### Studio area
- Occupies the lower portion of the main window.
- Persistent application navigation.
- Main content area with search, category filters and responsive media cards.
- Optional details/customization panel.
- Status and operation feedback.

## 3. Navigation

Primary destinations:
- Home
- Library
- Workshop
- Favorites
- Downloads
- Displays
- Settings
- Diagnostics

Library categories:
1. Pictures
2. Slideshow
3. Videos
4. Web Wallpapers
5. Anime Pictures
6. Anime Videos
7. 3D Wallpapers
8. Wallpaper Games

Workshop is for discovery, creator tools and package import; Library is for installed/local/available-to-run content. Do not conflate the two.

## 4. Category screen patterns

### Pictures
Responsive thumbnail grid, import folder/file, sort/filter, details, apply and favorite.

### Slideshow
Playlist editor with drag-and-drop order; interval, transition, shuffle, display target and preview controls.

### Videos
Video grid and preview player; duration, resolution, FPS, codec, mute, loop and playback controls.

### Web Wallpapers
Three-panel workspace: package/file tree, code editor, live preview. Include console, permissions, resource limits and validation status. Code editor must be a real editor component, not a decorative text box.

### Anime Pictures
Provider selector, search, tags, resolution filters, source attribution, preview, download and apply.

### Anime Videos
Video discovery cards, loop preview, duration/FPS/resolution metadata, download and apply.

### 3D Wallpapers
Scene library, central viewport, hierarchy/scene panel, inspector/properties, animation timeline and render-quality controls.

### Wallpaper Games
Launcher-style installed/discoverable game cards, game details, preview, controls, input configuration, save-state and resource policy.

## 5. Shared wallpaper details

A selected item opens a detail view with:
- Large preview
- Title and description
- Media type
- Author/source/license when known
- Resolution, dimensions, duration, format and file size when applicable
- Favorite
- Apply
- Customize
- Remove from library
- Display target

Do not invent metadata. Unknown values should be omitted or shown as unknown.

## 6. Design tokens — initial proposal

- Background: `#0B0B10`
- Surface 1: `#14131B`
- Surface 2: `#1D1A27`
- Border: `#343044`
- Primary: `#8B3DDB`
- Primary hover: `#A855F7`
- Accent: `#D946EF`
- Secondary accent: `#38BDF8`
- Text primary: `#F5F3FA`
- Text secondary: `#AAA5B8`
- Success: `#34D399`
- Warning: `#FBBF24`
- Error: `#FB7185`

Tokens are provisional and must be checked for contrast in the prototype.

## 7. Interaction principles

- Single click selects; explicit Apply action changes the desktop.
- Double click opens details or preview; it must not unexpectedly apply.
- Right-click offers contextual actions.
- Drag/drop supports import and playlist reordering.
- All asynchronous work shows progress and cancellation where safe.
- Destructive actions require confirmation or undo.
- Keyboard navigation and visible focus states are required.
- Empty, loading, error, offline and permission-denied states must be designed.
- UI layout must adapt to window resize and high-DPI displays.

## 8. Prototype screens required for Phase 0

1. Main shell with top desktop preview and Studio below
2. Home
3. Library overview and category filters
4. Wallpaper details/preview
5. Workshop/discovery
6. Pictures
7. Slideshow editor
8. Video library/player
9. Web wallpaper editor
10. Anime picture discovery
11. Anime video discovery
12. 3D editor
13. Wallpaper games launcher/details
14. Displays
15. Settings
16. Downloads and diagnostics

## 9. Responsive baseline

Target desktop window:
- Minimum: 1100 × 700 logical pixels, subject to prototype usability testing.
- Reference design target: 1440 × 900.
- Sidebar may collapse.
- Media grid adapts column count to available width.
- Avoid hard-coded 1600×900 composition for the new Studio UI.
