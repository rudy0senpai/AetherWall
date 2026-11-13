# Screen 2 — Home

## Layout

```
+------------------------------------------+
| Navigation | Search | [Home]              |
+------------------------------------------+
| Hero wallpaper (large preview)           |
| [Explore wallpapers →]                     |
+------------------------------------------+
| Quick start cards:                         |
| +------+ +------+ +------+ +------+      |
| | Pictures | | Slideshow | | Videos | ...|
| +------+ +------+ +------+ +------+      |
+------------------------------------------+
| Featured categories                        |
| +----+ +----+ +----+ +----+ +----+       |
| |Pics| |Anim| |Web | |3D  | |Games|     |
| +----+ +----+ +----+ +----+ +----+       |
+------------------------------------------+
| Recently applied / favorites strip        |
+------------------------------------------+
```

## Sections

1. **Hero:** large (16:9) featured wallpaper preview with gradient overlay
   and CTA.
2. **Quick-start:** 4-column responsive grid of category cards linking to each
   category's library.
3. **Featured categories:** horizontal scrolling category chips with counts.
4. **Recently applied / favorites:** compact wallpaper strip, favorites only.

## States

- Loading, offline (featured content unavailable), empty (no favorites).

## Notes

- The Home screen introduces the Studio; all exploration begins here.
- Discovery surface: pick a category or find content.
- No production playback on Home; preview only.
