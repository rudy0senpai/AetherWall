# Wallpaper Card

## Layout

- **Aspect ratio:** 16:9 (configurable per card size)
- **Border radius:** 12px
- **Background:** `var(--aw-surface-1)`
- **Border:** 1px solid `var(--aw-border)`
- **Hover:** border color brightens, lift shadow, cursor pointer

## Internal structure

```
[thumbnail 16:9]          [tall variant 2:1]
+------------------+       +------------------+
|                  |       |                  |
|  (cover)         |       |   (cover)        |
|                  |       |                  |
+------------------+       +------------------+
  title (14px)               title (14px)
  meta (12px)                  meta (12px)
  favourite button            favourite button
```

## States

- **Default:** neutral thumbnail, title visible, meta visible.
- **Hover:** thumbnail scale 1.05, border brightens, shadow `var(--aw-elevation-2)`.
- **Selected:** border 2px `var(--aw-primary)`, background `var(--aw-surface-2)`.
- **Favorited:** heart icon filled `var(--aw-accent)`.
- **Loading:** gradient placeholder.
- **Empty:** icon + "No wallpapers yet" + import button.
- **Error:** thumbnail placeholder + "Failed to load" + retry.

## Sizes

| Size | Thumbnail | Gap | Padding |
|---|---|---|---|
| Compact | 64×36 | 12px | 12px |
| Default | 96×54 | 16px | 16px |
| Large | 160×90 | 20px | 20px |

## Accessibility

- Role `article` with `aria-label` on the card.
- Favourite button has `aria-label`.
- Keyboard focusable; Enter/Space activate.
