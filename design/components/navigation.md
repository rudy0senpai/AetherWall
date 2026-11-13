# Navigation

## Layout

Persistent application navigation on the left side of the Studio area.

- **Width:** 154px (default); collapsible to 64px.
- **Position:** vertical, fixed, full height of Studio area.
- **Background:** `var(--aw-surface-1)`.
- **Border right:** 1px solid `var(--aw-border)`.

## Items

| Label | Icon | Route | Status |
|---|---|---|---|
| Home | house | /home | active |
| Library | grid | /library | active |
| Workshop | wrench | /workshop | active |
| Favorites | star | /favorites | active |
| Downloads | download | /downloads | active |
| Displays | monitors | /displays | active |
| Settings | gear | /settings | active |
| Diagnostics | bug | /diagnostics | active |

## Categories

Below the primary nav: 8 category chips:

1. Pictures
2. Slideshow
3. Videos
4. Web Wallpapers
5. Anime Pictures
6. Anime Videos
7. 3D Wallpapers
8. Wallpaper Games

## States

- **Active:** background `var(--aw-surface-2)`, left border 3px `var(--aw-primary)`.
- **Hovered:** background `var(--aw-surface-2)`, text `var(--aw-text-primary)`.
- **Collapsed:** labels hidden, icons only, wider tap targets.
- **Focus:** visible focus ring.

## Accessibility

- Nav is a `<nav>` landmark.
- Items are links/buttons in a list; visible focus order.
- Collapse toggle is a button with `aria-expanded`.
