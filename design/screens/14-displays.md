# Screen 14 — Displays

## Layout

```
+------------------------------------------+
| Navigation | Search | [Displays]           |
+------------------------------------------+
| Display list                              |
| [monitor card] [monitor card] [monitor]    |
+------------------------------------------+
| Each card:                                   |
| +--------------+                            |
| | [resolution]|  [name] | [primary] [x]   |
| | [port info]  |         | [scale] [Hz]   |
| +--------------+                            |
+------------------------------------------+
| Toolbar: [Refresh] [Add display]            |
+------------------------------------------+
```

## Features

- List of detected displays with resolution, name, port, scale factor,
  refresh rate, primary flag.
- Refresh topology button.
- Per-display wallpaper assignment is managed by the Studio; the display
  management screen is where displays are configured.
- Multiple monitor support with position mapping.

## States

- Loading, no displays found (scan button), error, permission denied.
