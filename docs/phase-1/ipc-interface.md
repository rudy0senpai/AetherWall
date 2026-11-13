# Phase 1 — IPC Interface Contract

**Date:** 2026-11-13
**Contract version:** 1.0

## 1. Overview

This document defines the D-Bus interface between the AetherWall Studio (Qt 6/QML
UI) and the AetherWall Daemon (Rust background service).

- **Service name:** `org.aetherwall.daemon.1`
- **Object path:** `/org/aetherwall/daemon/1`
- **Interface name:** `org.aetherwall.daemon.1`

All method calls use strongly-typed request/response structures. The contract is
versioned; clients must send `client_max_version` and the daemon must respond
with a compatible version.

## 2. Message format

### 2.1 Request

```json
{
  "client_max_version": "1.0",
  "method": "GetApplicationStatus",
  "serial": 42,
  "payload": {}
}
```

### 2.2 Response

```json
{
  "serial": 42,
  "success": true,
  "payload": {
    "pid": 1234,
    "state": "running",
    "supported_runtimes": ["image", "slideshow"],
    "database_ready": true
  }
}
```

### 2.3 Error response

```json
{
  "serial": 42,
  "success": false,
  "error_name": "WallpaperNotFound",
  "error_message": "Wallpaper with id 'unknown-123' not found"
}
```

## 3. Method reference

| Method | Request | Response | Description |
|---|---|---|---|
| `GetApplicationStatus` | `{ }` | `StatusResponse` | Application status |
| `GetAvailableDisplays` | `{ }` | `DisplaysResponse` | List of detected displays |
| `GetWallpaperLibrary` | `{ type?, query? }` | `LibraryResponse` | Wallpaper library list |
| `ImportWallpaper` | `{ path }` | `ImportResponse` | Import a wallpaper file |
| `RemoveWallpaper` | `{ id }` | `ImportResponse` | Remove a wallpaper |
| `SetActiveWallpaper` | `{ wallpaper_id, display_id }` | `{ success: bool }` | Apply wallpaper to display |
| `StopActiveWallpaper` | `{ }` | `{ success: bool }` | Stop current wallpaper |
| `GetCurrentWallpaper` | `{ }` | `LibraryItem` | Current active wallpaper |
| `UpdateApplicationSettings` | `{ key, value }` | `{ success: bool }` | Update a setting |
| `GetRuntimeErrors` | `{ }` | `ErrorResponse` | Retrieve recent runtime errors |

## 4. Error handling

Every error response includes `error_name` (a stable string) and
`error_message` (human-readable). The Studio must not interpret error names
across versions; use them for logging and diagnostics.

### Standard error names

| Error name | Meaning |
|---|---|
| `ServiceUnavailable` | Daemon is not running |
| `ConnectionFailed` | D-Bus connection failed |
| `InvalidInput` | Invalid request payload |
| `WallpaperNotFound` | Referenced wallpaper id does not exist |
| `UnknownDisplay` | Referenced display id does not exist |
| `Forbidden` | Permission denied |
| `RuntimeFailed` | Wallpaper runtime crashed |

## 5. Async operations

Long-running operations (import, apply) are performed asynchronously. The
daemon returns immediately with a pending response; the Studio may poll
`GetRuntimeErrors` or subscribe to `DaemonEvent` notifications for progress.

### Events (signal emissions)

| Signal | Payload | Description |
|---|---|---|
| `LibraryItemAdded` | `{ id: string }` | New wallpaper imported |
| `LibraryItemUpdated` | `{ id: string }` | Existing wallpaper updated |
| `LibraryItemRemoved` | `{ id: string }` | Wallpaper removed |
| `ImportProgress` | `{ path, bytes_received, bytes_total }` | Import progress |
| `SessionStateChanged` | `{ session_id, state }` | Wallpaper session state change |
| `DisplayTopologyChanged` | `{ }` | Display topology changed |
| `ErrorReported` | `{ error_code, error_message }` | Runtime error |

## 6. Service availability detection

The Studio must:

1. Attempt to connect to the D-Bus session bus.
2. If the daemon is not found, show a "Daemon unavailable" error state.
3. Retry connection periodically (every 5 seconds) until the daemon starts.
4. On connection loss, reconnect and reconcile active sessions.

## 7. Threading and UI responsiveness

All D-Bus calls must be made from a background thread / async context.
The QML UI must never block on a D-Bus call:

- Use Qt's `QDBusPendingCall` or `async`/`await` D-Bus wrappers.
- Show loading indicators during long-running operations.
- Update progress via signals, not by blocking the UI thread.
