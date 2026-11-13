# Screen 8 — Video Library / Player

## Layout

```
+------------------------------------------+
| Navigation | Search | [Videos]            |
+------------------------------------------+
| Media grid with embedded preview          |
| [card] [card] [card] [card] [card] [card] |
+------------------------------------------+
| Filters: duration | resolution | codec    |
+------------------------------------------+
```

## Card content

- Thumbnail (frame at timestamp 00:01).
- Title + duration + resolution + FPS + codec metadata.
- Actions: play, details, apply.

## Player (detail view)

- Muted by default.
- Controls: play/pause, seek, loop, playback speed, hardware decode toggle.
- Expose codec, resolution, duration, FPS metadata.

## States

- Loading grid, empty, no videos found, error, offline, player error.
