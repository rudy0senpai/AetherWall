# Dialog

## Layout

- **Overlay:** fixed, inset 0, background `rgba(0,0,0,0.6)`, backdrop-filter Gaussian blur.
- **Panel:** fixed max-width 520px, max-height 85vh, background `var(--aw-surface-1)`,
  border 1px `var(--aw-border-strong)`, border-radius 16px, shadow `var(--aw-elevation-3)`.

## Internal structure

```
+----------------------------------+
|  [close]                    X    |
+----------------------------------+
|  title (2xl)                     |
|  description (sm)                  |
|                                    |
|  [content area]                    |
|                                    |
|  [cancel]  [confirm]              |
+----------------------------------+
```

## Variants

- **Alert:** error icon, destructive confirm button.
- **Confirm:** warning icon, confirm/cancel.
- **Input:** form fields with labels, optional validation state.
- **Info:** informational content, single confirm.
- **Full screen:** on tablet, replaces content area.

## Accessibility

- Role `dialog` with `aria-modal="true"`.
- `aria-labelledby` on title, `aria-describedby` on description.
- Focus trapped inside dialog; focus returns to trigger on close.
- Close by Escape or × button.
