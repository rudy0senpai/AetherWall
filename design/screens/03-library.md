# Screen 3 — Library Overview and Category Filters

## Layout

```
+------------------------------------------+
| Navigation | Search | [Library]           |
+------------------------------------------+
| Category filter bar (horizontal scroll)  |
| [Pics] [Slideshow] [Videos] [Web] ...    |
+------------------------------------------+
| Media grid (responsive)                  |
| [card] [card] [card] [card] [card]       |
| [card] [card] [card] [card] [card]       |
+------------------------------------------+
| View toggle: list | grid                 |
+------------------------------------------+
```

## Features

- **Category filter bar:** horizontal scroll of the 8 categories; active state
  filters the grid.
- **Search:** global search bar above filter bar.
- **View toggle:** grid (default) / list.
- **Result count:** "128 wallpapers" text.
- **Pagination or infinite scroll** with "Load more" state.

## States

- Loading (skeleton cards), empty (no wallpapers in category),
  error (load failed), offline.

## Accessibility

- Filter buttons are buttons; active state is indicated by aria-pressed and
  visible style.
- Grid is a list; each card is a link/button.
