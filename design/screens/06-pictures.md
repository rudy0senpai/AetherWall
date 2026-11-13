# Screen 6 — Pictures Library

## Layout

```
+------------------------------------------+
| Navigation | Search | [Pictures]          |
+------------------------------------------+
| Filter bar: [folder selector] [reset]     |
+------------------------------------------+
| Media grid (responsive)                   |
| [card] [card] [card] [card] [card] [card] |
+------------------------------------------+
| Sort: [newest | oldest | name | size]    |
| View: [grid] [list]                       |
+------------------------------------------+
```

## Features

- Import folder/file: pick from disk; index supported image formats.
- Sort: newest/oldest/name/size; default newest.
- Pagination/infinite scroll.
- Empty state: "No wallpapers in this folder" + import button.
- Details show resolution, file size, mode (fit/fill/crop/center/stretch).
- Apply and customize.

## States

- Loading, empty, error, offline, import in progress, import success,
  import failure.
