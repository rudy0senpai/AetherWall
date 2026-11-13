# Screen 16 — Downloads and Diagnostics

## Layout

```
+------------------------------------------+
| Navigation | Search | [Downloads/Settings] |
+------------------------------------------+
| Downloads list                            |
| [title] [status badge] [progress] [cancel]|
| [title] [status badge] [progress] [cancel]|
| [title] [status badge]                    |
+------------------------------------------+
| Diagnostics:                              |
| +-----------------------------+            |
| | Log viewer                  |            |
| | [scrollable log lines]      |            |
| |                             |            |
| | Runtime status              |            |
| | [image runtime] [running]   |            |
| | [mpv runtime] [idle]        |            |
| | [web runtime] [crashed]     |            |
| +-----------------------------+            |
+------------------------------------------+
```

## Downloads

- List of active and completed downloads.
- Status: pending, downloading, paused, failed, completed.
- Progress bar, cancel button.
- Filter: all, downloading, failed, completed.

## Diagnostics

- Log viewer: system and runtime logs, scrollable, clear button.
- Runtime status: per-runtime health, last error, last health check.
- Event log: library changes, display topology changes, provider progress.

## States

- Loading, no events, log empty, offline.
