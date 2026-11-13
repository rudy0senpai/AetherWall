# Category Card

## Layout

- **Rounded:** 12px
- **Background:** `var(--aw-surface-1)`
- **Border:** 1px solid `var(--aw-border)`
- **Hover:** border brightens, shadow lift

## Structure

```
+------------------+
|  [icon]  Title   |
|  subtitle        |
+------------------+
  count badge (top right)
```

## States

- Default, hover (lift + border brighten), selected (border 2px primary).
- Disabled (opacity 0.5).
- Loading, empty, error states same as wallpaper card.

## Use

- Category filter chips.
- Category discovery cards on Home and Workshop.
- Category list on Library overview.
