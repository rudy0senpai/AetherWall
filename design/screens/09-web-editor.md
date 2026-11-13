# Screen 9 — Web Wallpaper Editor

## Layout

```
+------------------------------------------+
| Navigation | Search | [Web Wallpapers]    |
+------------------------------------------+
| Three-panel workspace:                    |
| +---------------------------+  +--------+ |
| | Package / file tree       |  | Editor | |
| | - index.html              |  | (code) | |
| | - style.css               |  |        | |
| | - script.js               |  |        | |
| | - scene.glb               |  +--------+ |
| | - wallpaper.json          |  +--------+ |
| +---------------------------+  | Preview | |
|                               | (live)  | |
|                               +--------+---+
+------------------------------------------+
| Console | Permissions | Resource limits   |
| Validation status                       |
+------------------------------------------+
```

## Panels

1. **File tree:** package structure, new file creation, delete, rename.
2. **Code editor:** real editor component (not decorative text). Syntax
   highlighting for HTML/CSS/JavaScript. Line numbers. Multiple cursors.
3. **Live preview:** renders the wallpaper; resizes with the window;
   developer console accessible via keyboard shortcut.
4. **Side panel:** console, permissions display, resource limits, validation
   status.

## States

- Loading, code changed (dirty indicator), validation error, console open,
  no preview (fallback placeholder).
