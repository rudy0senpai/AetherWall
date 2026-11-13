# Reliability and Security — AetherWall

## 1. Purpose

This module implements reliability, security, and safety guarantees for the
AetherWall application. Wallpaper content must never be treated as trusted
executable code.

## 2. Input validation

- All user-supplied input is validated against allowlists where possible.
- File paths are validated for traversal (`..`).
- File types are validated by extension and magic bytes.
- URLs are validated by scheme (http/https only).

## 3. Safe filesystem operations

- All file operations use absolute paths resolved via `std::path::Path::canonicalize`.
- Paths are checked for existence before use.
- Symlinks are resolved and checked for traversal.
- File writes use atomic rename (write to temp file, then `rename`).

## 4. Path traversal protection

- Archive extraction rejects any path containing `..` or absolute paths.
- Package manifests are validated against the schema.
- User-provided directory paths are cleaned and validated.

## 5. File type validation

- Image files are validated by magic bytes (not just extension).
- Video files are validated against supported container/codec lists.
- Executable files (`.exe`, `.dll`, `.so`, `.bin`, etc.) are rejected.

## 6. Runtime isolation boundaries

- Runtimes run in separate processes with resource limits.
- Web/game content runs in a constrained HTML5 runtime.
- Native image/video runtimes use mature, sandboxed libraries.
- No arbitrary code execution from wallpaper content.

## 7. Resource cleanup

- Runtime processes are terminated on session end.
- Temp files and thumbnails are cleaned periodically.
- Memory is freed on runtime stop.
- File handles are closed on disposal.

## 8. Process lifecycle management

- Daemon tracks all runtime child processes.
- On daemon stop, all child processes are terminated.
- Stale processes are detected and cleaned on daemon start.
- Process crashes are reported as runtime errors.

## 9. Diagnostic logs

- Structured logging via `tracing` (trace, debug, info, warn, error).
- Log levels configurable via `DaemonConfig.log_level`.
- All D-Bus calls logged with serial and method name.
- Session state changes logged.
- Runtime health checks logged.

## 10. Graceful failure handling

- Runtime failures do not crash the daemon.
- The daemon marks the session as failed and records diagnostics.
- The Studio is notified via D-Bus error or event.
- The user is presented with a retry/fallback option.
- No automatic destructive repair is performed.

## 11. Security principles

1. Wallpaper content is untrusted.
2. No native executable payloads from packages.
3. Web/game content sandboxed.
4. Network access opt-in and per-feature.
5. Secrets stored in keyring, not SQLite.
6. No secrets in logs.
