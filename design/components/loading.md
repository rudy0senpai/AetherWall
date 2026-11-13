# Loading State

## Layout

- **Indicator:** centered spinner (24px), 600ms CSS rotation.
- **Label:** "Loading…" secondary text, 14px, `var(--aw-text-secondary)`.
- **Background:** `var(--aw-surface-1)` (transparent unless inside a card).

## Variants

| Variant | Description |
|---|---|
| Skeleton | Placeholder block with animated gradient shimmer |
| Spinner | Circular spinner with label |
| Shimmer | Thin gradient line for meta fields |

## Colors

- Spinner: `var(--aw-primary)` with gradient stroke.
- Skeletal shimmer: `var(--aw-surface-3)` over background, animate opacity 1s.

## Accessibility

- `role="status"` / `aria-live="polite"` for the label.
- Spinner has `aria-label="Loading"`.
