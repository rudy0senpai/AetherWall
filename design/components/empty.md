# Empty State

## Layout

- **Icon:** 64px, `var(--aw-surface-3)` with border 1px `var(--aw-border)`.
- **Title:** 2xl, `var(--aw-text-primary)`.
- **Description:** 14px, `var(--aw-text-secondary)`.
- **Action:** primary button, 16px padding, 8px radius.

## States

| State | Icon | Title | Action |
|---|---|---|---|
| Import folder | folder | "No wallpapers in this folder" | "Import folder" |
| No search results | search | "No matches for this search" | "Clear search" |
| Library empty | library | "Your library is empty" | "Import wallpapers" |
| No internet | cloud-off | "You're offline" | "Check connection" |
| No favorites | star | "No favorites yet" | "Discover wallpapers" |
| No downloads | download | "No downloads yet" | "Import or discover" |
| No display detected | monitor | "No displays found" | "Scan for displays" |

## Accessibility

- Icon has `role="img"` and aria-label.
- Action has descriptive `aria-label`.
