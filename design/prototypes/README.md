# Prototypes — AetherWall Phase 0

Interactive, high-fidelity screen prototypes built with HTML/CSS using the
design system tokens in `design/design-system/`.

## Structure

- `prototype.html` — single-file interactive prototype carrying all 16 screens
  as navigable views.
- `prototype-spec.md` — prototype scope, limitations and validation notes.

## Prototype catalogue

Each screen in the prototype corresponds to one of the 16 required screens:

| # | Screen | Prototype |
|---|---|---|
| 1 | Main shell with top desktop preview and Studio below | `screen-01-mesh` |
| 2 | Home | `screen-02-home` |
| 3 | Library overview and category filters | `screen-03-library` |
| 4 | Wallpaper details/preview | `screen-04-details` |
| 5 | Workshop/discovery | `screen-05-workshop` |
| 6 | Pictures | `screen-06-pictures` |
| 7 | Slideshow editor | `screen-07-slideshow` |
| 8 | Video library/player | `screen-08-videos` |
| 9 | Web wallpaper editor | `screen-09-web-editor` |
| 10 | Anime picture discovery | `screen-10-anime-pics` |
| 11 | Anime video discovery | `screen-11-anime-vids` |
| 12 | 3D editor | `screen-12-3d-editor` |
| 13 | Wallpaper games launcher/details | `screen-13-games` |
| 14 | Displays | `screen-14-displays` |
| 15 | Settings | `screen-15-settings` |
| 16 | Downloads and diagnostics | `screen-16-downloads` |

## Limitations

- Prototype uses static sample content and gradient placeholders, not real
  media assets or production runtimes.
- Desktop preview is a clearly-labelled preview snapshot, not a live
  compositor-rendered wallpaper.
- No Qt/QML runtime; this is a UI/UX prototype for design validation only.
- Production playback code is out of scope for Phase 0.
