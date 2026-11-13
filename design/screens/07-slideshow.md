# Screen 7 — Slideshow Editor

## Layout

```
+------------------------------------------+
| Navigation | Search | [Slideshow]         |
+------------------------------------------+
| Playlist editor (drag-and-drop)           |
| +--------------------------------------+  |
| | card  card  card  card  card  card   |  |
| | [-1] [-2] [-3] [-4] ...                |  |
| +--------------------------------------+  |
+------------------------------------------+
| Controls: [shuffle] [interval] [transition]|
| [start] [pause] [stop] [clear]            |
+------------------------------------------+
| Preview: apply to selected display then    |
| [preview] [apply]                         |
+------------------------------------------+
```

## Features

- Drag-and-drop reorder playlist items.
- Interval (ms) and transition type controls.
- Shuffle mode: sequential / shuffled / random.
- Per-display assignment.
- Preview before apply.
- Import from folder/collection button.

## States

- Empty playlist (import), loading, drag placeholder, apply pending,
  runtime failure.

## Notes

- Slideshow is a scheduling runtime; content is imported images.
- No production playback in Phase 0 prototypes.
