# Error State

## Layout

- **Icon:** error icon (24px), `var(--aw-error)`.
- **Title:** 2xl, `var(--aw-error)`.
- **Description:** 14px, `var(--aw-text-secondary)`.
- **Retry button:** primary button, 16px padding.

## Variants

| State | Color | Use |
|---|---|---|
| Network error | `var(--aw-error)` | Provider offline, fetch failure |
| Validation error | `var(--aw-error)` | Malformed package, missing fields |
| Runtime error | `var(--aw-error)` | Runtime crashed, playback failed |
| Permission denied | `var(--aw-warning)` | Restricted capability |
| Generic | `var(--aw-border-strong)` | Unexpected failure |

## Accessibility

- `role="alert"` on the error container.
- Error message is programmatically associated with the field that failed.
