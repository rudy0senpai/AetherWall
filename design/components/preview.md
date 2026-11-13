# Preview Component

## Layout

- **Default aspect ratio:** 16:9 (responsive; also 2:1 for tall, square for cards).
- **Border radius:** 12px
- **Background:** `var(--aw-surface-2)`
- **Border:** 1px solid `var(--aw-border)`
- **Shadow:** `var(--aw-elevation-1)` default, `var(--aw-elevation-2)` on hover/selected

## Internal structure

```
+----------------------------+
|                            |
|   (image/video/web/viewer) |
|                            |
+----------------------------+
  aspect-ratio shim
```

## States

- **Loaded:** content fills frame; object-fit `cover`.
- **Playing:** playback controls overlay (pause/play, progress).
- **Paused:** static image.
- **Error:** fallback placeholder + retry.
- **No preview:** gradient placeholder.

## Variants

| Variant | Usage |
|---|---|
| Thumbnail | 64×36 / 96×54 / 160×90 |
| Detail preview | 400×225 min, 800×450 max |
| Wallpaper preview | Full desktop preview (top of shell) |
| Editor preview | Split-panel live preview |

## Accessibility

- `role="img"` with descriptive aria-label.
- Controls have visible focus.
