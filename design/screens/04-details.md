# Screen 4 — Wallpaper Details / Preview

## Layout

```
+------------------------------------------+
| Navigation | Search |                   <- back |
+------------------------------------------+
| [detail preview - large]                  |
| +---------------------------+             |
| | image/video/web preview   |             |
| +---------------------------+             |
+------------------------------------------+
| Title ____________________________________|
| Description ______________________________|
| Media type [image | video | web | 3D | ...] |
| Metadata: resolution | duration | file size |
| Author / source / license                 |
+------------------------------------------+
| Actions: [Apply to display] [Customize]   |
|          [Favorite] [Remove] [Share]      |
+------------------------------------------+
```

## Sections

1. **Large preview:** shows the selected media type; web wallpapers show the
   live preview; videos show the current frame; 3D shows a first-person view.
2. **Metadata:** title, description, media type badge, resolution, duration,
   file size, author/source/license.
3. **Actions:** apply to display, customize, favorite, remove, share.

## States

- Loading preview, preview error, no preview (gradient placeholder).
- Detail of a downloaded/local item; provider item shows "source: provider".
- Customize panel opens inside the detail view.

## Notes

- Unknown metadata is omitted or shown as "Unknown".
- Apply does not auto-apply; it opens a display selection dialog.
