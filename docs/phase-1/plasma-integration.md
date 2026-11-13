# Phase 1 — KDE Plasma 6 Integration

**Date:** 2026-11-13
**Branch:** `phase-1/core-implementation`

## 1. Overview

This document records the investigation and design of KDE Plasma 6 wallpaper
integration, prioritising **Wayland** with **X11 as a compatibility path**
(ADR-004).

> **Status: Unvalidated.** This environment runs Windows 11; no KDE Plasma 6
> installation is available. The integration mechanism is designed as a safe
> adapter boundary — the exact API calls must be validated on a KDE Plasma 6
> host before production use.

## 2. Plasma wallpaper plugin architecture

KDE Plasma provides two main mechanisms for showing wallpapers on a display:

### 2.1 Plasma Wallpaper Plugin (Wayland & X11)

The standard mechanism is a **Plasma Wallpaper Plugin** — a QML component that
implements the `org.kde.PlasmaWallpaper` interface. The plugin:

- Is loaded per-display by the Plasma compositor.
- Receives display geometry (width, height, scale factor).
- Runs for the lifetime of the user session.
- Can be enabled/disabled per user.

Required plugin QML file structure:

```qml
// Wallpaper Plugin (e.g. aetherwall-wallpaper/contents/config/main.xml)
<?xml version="1.0" encoding="UTF-8"?>
<component>
    <Name>AetherWall Wallpaper</Name>
    <Metadata>aetherwall-wallpaper/metadata.json</Metadata>
    <Script>aetherwall-wallpaper/contents/code/contents.js</Script>
</component>
```

The plugin code (`contents.js`) receives the display and must apply the wallpaper:

```javascript
// contents.js — Plasma wallpaper plugin entry point
function load()
{
    // imagePath is provided by the daemon via DBus
    if (imagePath) {
        // Apply the wallpaper image
        image.source = imagePath;
    }
}

function update()
{
    if (imagePath) {
        image.source = imagePath;
    }
}

function unload()
{
    // Cleanup
}
```

### 2.2 How the daemon communicates with the plugin

The daemon cannot directly control a Plasma plugin across the Wayland
compositor. The integration workflow is:

1. User selects a wallpaper in the AetherWall Studio.
2. Daemon sends the wallpaper path over D-Bus (`org.aetherwall.daemon.1.SetActiveWallpaper`).
3. Daemon (or a background companion service) writes the path to a
   well-known location (e.g. `$XDG_RUNTIME_DIR/aetherwall/current-wallpaper`).
4. The Plasma wallpaper plugin polls (or is notified via D-Bus) of the path.
5. The plugin reads the path and applies the image using its internal image
   loading mechanism (`Image` or `qaitoring` APIs).

### 2.3 QML wallpaper plugin appropriateness

**Yes**, a QML wallpaper plugin is the correct approach:

- Plasma's wallpaper system is QML-based.
- QML provides the image loading, scaling, and animation primitives.
- The plugin is declarative and safe for the desktop environment.

## 3. Wayland vs X11

| Aspect | Wayland | X11 |
|---|---|---|
| Primary plugin surface | `org.kde.PlasmaWallpaper` | `org.kde.PlasmaWallpaper` |
| Daemon-to-plugin communication | Companion helper + file/DBus polling | File/DBus polling |
| Display geometry | Provided by compositor | Provided by Xinerama/XRandr |
| Screenshots / content | Compositor restricts | Direct access |
| Validation | Requires Plasma 6 Wayland | Requires Plasma 6 X11 |
| Priority | ✅ Primary target | Compatibility path |

## 4. Daemon-to-compositor communication

### 4.1 Recommended architecture

```
Studio → D-Bus → Daemon → writes wallpaper path
                              ↓
                 Companion helper (daemon-owned)
                              ↓
        Polls filesystem or D-Bus signal
                              ↓
           Plasma wallpaper plugin reads path
                              ↓
                Applies image to display
```

### 4.2 Per-display assignment

- The daemon maintains a per-display mapping: `display_id → wallpaper_id`.
- This mapping is persisted in SQLite (`display_assignments` table).
- On daemon restart, the mapping is reloaded and applied to the plugins.
- Display topology changes (monitor addition/removal) trigger a
  `DisplayTopologyChanged` event; the daemon reconciles and re-applies.

## 5. Login, logout, and restart behaviour

| Event | Behaviour |
|---|---|
| Login | Daemon starts; existing session assignments restored; plugins re-applied. |
| Logout | All wallpaper sessions stop; plugin state cleaned. |
| Daemon restart | Sessions reconciled; active wallpapers re-applied if still present. |
| Plasma restart | Plugins are reloaded; daemon re-applies active assignments. |

## 6. Limitations and open questions

1. **Wayland content access.** The Wayland compositor restricts direct content
   access. The Plasma plugin must load images using safe, compositor-approved
   surfaces. This is validated on a Plasma 6 host.
2. **Daemon restart concurrently with a wallpaper running.** The daemon may
   restart during an active wallpaper session. The plugin must handle this
   gracefully: re-read the wallpaper path from the companion service.
3. **Session persistence across daemon stop/start.** The daemon is a per-user
   service, not a system daemon. On logout, all sessions stop and must be
   explicitly saved or discarded.
4. **Plasma version differences.** Plasma 5 vs Plasma 6 differ in the API.
   This product targets Plasma 6 only; Plasma 5 compatibility is deferred.
5. **Alternative: `org.kde.plasmashell` scripting.** The daemon could also
   control the wallpaper via the PlasmaShell scripting interface, but this is
   less reliable under Wayland and requires a running Plasma session.

## 7. Safe adapter boundary

Until the integration is validated on a Plasma 6 host, the daemon implements
a **safe adapter boundary**:

- The daemon writes the wallpaper path to a defined location.
- The plugin polls a well-known file or D-Bus signal.
- If the plugin cannot read the path, no error is reported to the user;
  the wallpaper simply does not appear.
- No crash, no fallback failure.

This ensures that an unimplemented or unvalidated Plasma integration cannot
crash the daemon or the Studio.
