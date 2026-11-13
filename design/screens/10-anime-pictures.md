# Screen 10 — Anime Picture Discovery

## Layout

```
+------------------------------------------+
| Navigation | Search | [Anime Pictures]    |
+------------------------------------------+
| Provider selector                         |
| [All providers] [Anime Land][LoveLive][...]|
+------------------------------------------+
| Filters: tags | resolution | media type   |
+------------------------------------------+
| Discovery cards (large thumbnail + title) |
| [card] [card] [card] [card] [card] [card] |
+------------------------------------------+
```

## Features

- Provider selector (enable/disable per provider).
- Search, tags, category, resolution, media-type filters.
- Source attribution displayed on each card.
- Download/cache flow; preview then apply.
- Provider failure shows provider-specific error; local library preserved.

## States

- Loading providers, no providers, offline, no results, provider error,
  download in progress, permission denied.
