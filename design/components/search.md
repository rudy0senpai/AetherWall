# Search

## Layout

- **Height:** 44px (min)
- **Border radius:** 8px
- **Background:** `var(--aw-surface-1)`
- **Border:** 1px solid `var(--aw-border-strong)`
- **Padding:** horizontal 16px, vertical 12px

## Structure

```
[search icon] [input field]                [clear button]
```

## States

- **Default:** subtle border.
- **Focus:** border brightens, shadow `var(--aw-elevation-1)`.
- **Has value:** × clear button appears.
- **Loading:** shimmer inside input.
- **Error:** red border, error message below.

## Placement

- **Global search bar** in the top-right of the Studio area.
- **Filter sidebar:** collapsible search in Library and Workshop.
- **Category page search:** filter bar above media grid.

## Accessibility

- Label associated with input via `for`/`id`.
- Input has `aria-label="Search..."`.
- Enter submits; arrow keys navigate results.
