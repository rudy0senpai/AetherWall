# Button

## States

- Default
- Hover
- Pressed / active
- Disabled
- Loading
- Destructive

## Spec

- **Height:** 36px (min)
- **Padding horizontal:** 16px
- **Border radius:** 8px
- **Font:** Inter, semibold, 14px
- **TextColor:** primary on dark surface; inverse white on accent
- **Shadow:** none; use subtle border for outline variant

## Variants

| Variant | Background | Border | Use |
|---|---|---|---|
| Primary | `var(--aw-primary)` | none | Primary action |
| Primary hover | `var(--aw-primary-hover)` | none | — |
| Ghost | transparent | 1px `var(--aw-border)` | Secondary action, no fill |
| Outline | `var(--aw-surface-2)` | 1px `var(--aw-border-strong)` | Emphasised secondary |
| Destructive | `var(--aw-error)` tinted | 1px `var(--aw-error)` | Destructive action |
| Icon | `var(--aw-surface-2)` | 1px `var(--aw-border-strong)` | Action icon button |

## Interactions

- Hover: background lightens, border color brightens.
- Pressed: background darkens 6%.
- Loading: replaces label with spinner, preserves width, disables hover feedback.
- Disabled: opacity 0.5, cursor not-allowed, no pointer events.

## Accessibility

- Minimum touch target 40×40px.
- Visible focus ring: `outline: 2px solid var(--aw-accent); outline-offset: 2px`.
- Icon-only buttons include `aria-label`.
