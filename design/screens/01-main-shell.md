# Screen 1 — Main Shell

## Layout

```
+------------------+------------------------------------------+
|  Desktop preview |  Studio area                               |
|  (top section)   |  +------------------------------------+ |
|                  |  | Navigation | Content            | |
|  [display sel]   |  | [Home][Lib][Workshop][Fav][Down] | |
|  [size ctrl]     |  | [Search ..........]            | |
|                  |  | +----------------------------+   | |
|  wallpaper mip   |  | |                              |   | |
|  [apply btn]     |  | |      media grid / page       |   | |
|                  |  | |                              |   | |
|                  |  | +----------------------------+   | |
|                  |  | [status bar / footer]          | |
+------------------+------------------------------------------+
```

## Regions

- **Desktop preview (top):** 220px–280px tall. Shows the selected display's
  current wallpaper. Includes: display selector (combo), preview size control,
  apply buttons, and a live/preview status indicator. Must NOT include any
  system-monitoring HUD, CPU/RAM/battery gauges, graphs, or telemetry cards.

- **Studio area (lower):** flexible height. Persistent left navigation.
  Main content area with search, category filters, and responsive media cards.
  Optional details/customization panel.

## States

- **Loading:** skeleton placeholders in both preview and Studio.
- **Empty:** "No display detected" or "No wallpaper applied" states.
- **Error:** preview load failed → fallback snapshot + retry.

## Constraints

- Minimum window 1100×700 logical px; reference design target 1440×900.
- Window resizes; navigation collapses; media grid adapts columns.
- High-DPI aware.

## Notes

- Desktop preview is a preview surface, not a system-monitoring overlay.
- No telemetry cards, no graphs, no monitoring dock.
