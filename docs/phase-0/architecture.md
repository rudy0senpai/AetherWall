# Technical Architecture

## 1. High-level model

```text
┌──────────────────────────────────────────────────────┐
│                 AetherWall Studio                    │
│                 Qt 6 / QML UI                        │
│  Desktop Preview | Library | Workshop | Settings     │
└─────────────────────────┬────────────────────────────┘
                          │ versioned D-Bus API
┌─────────────────────────▼────────────────────────────┐
│                 AetherWall Daemon                    │
│  Session | Scheduler | Library | Display | Settings  │
│  Runtime Supervisor | Package Manager | Event Bus    │
└──────────────┬──────────────┬──────────────┬──────────┘
               │              │              │
       ┌───────▼──────┐ ┌─────▼──────┐ ┌─────▼─────────┐
       │ Image/Slide  │ │ libmpv     │ │ Qt WebEngine  │
       │ Runtime      │ │ Runtime    │ │ Runtime       │
       └──────────────┘ └────────────┘ └───────────────┘
               │              │              │
       ┌───────▼──────┐ ┌─────▼──────────────┐
       │ wgpu + glTF  │ │ HTML5 Game Runtime │
       │ 3D Runtime   │ │ constrained        │
       └──────────────┘ └────────────────────┘
                          │
              KDE Plasma integration backend
                    Wayland first / X11
```

## 2. Components

### Studio UI
- Qt 6 / QML.
- Owns navigation, forms, previews, user interaction and presentation state.
- Calls daemon APIs; does not directly manage long-running playback processes.
- Must not block on disk, network, decode or render operations.

### Core and daemon
- Rust.
- Owns wallpaper catalog, active sessions, scheduling, display mapping, settings, runtime supervision and event delivery.
- Persists state through SQLite.
- Exposes a versioned D-Bus contract.
- Runs as a per-user service, not a privileged system daemon.

### Runtime adapters
- Image/slideshow: native image pipeline and scheduling.
- Video: libmpv.
- Web: Qt WebEngine with restricted profile and permissions.
- 3D: wgpu and glTF/GLB.
- Games: constrained HTML5 runtime with stricter input and resource policy.
- Anime media: uses image/video runtimes after provider content is imported or cached; online discovery itself is not a renderer.

### Desktop backend
- KDE Plasma 6 is the first-class integration target.
- Wayland is the priority.
- X11 support is a separate compatibility path.
- Backend capability detection must identify supported wallpaper surfaces, monitor geometry, focus/fullscreen signals and lifecycle events.
- Do not assume generic Wayland permits arbitrary desktop-background windows. Use the supported Plasma integration mechanism and document compositor-specific constraints.

## 3. Runtime lifecycle contract

Every runtime adapter must implement equivalent operations:

- `validate(package_or_asset)`
- `prepare(wallpaper_id, display_target, settings)`
- `start(session_id)`
- `pause(session_id, reason)`
- `resume(session_id)`
- `update_settings(session_id, patch)`
- `stop(session_id)`
- `health(session_id)`
- `dispose(session_id)`

Operations are asynchronous and return structured errors. Runtime-specific data must not leak into the shared catalog model.

## 4. Failure isolation

- Studio UI, daemon and renderer are separate failure domains.
- Runtime process crash: daemon marks session failed, records diagnostics and offers retry/fallback.
- Daemon restart: Studio reconnects; active sessions are reconciled.
- Provider failure: show provider-specific error; preserve local library.
- Database migration failure: stop safely and preserve the previous database file.
- No automatic destructive repair.

## 5. IPC and event model

Use D-Bus with explicit interface versioning. Document method names, input/output types, error names and event payload schemas before implementation.

Events include:
- Library item added/updated/removed
- Import progress/completion/failure
- Wallpaper session state changes
- Display topology changes
- Provider search/download progress
- Runtime health and diagnostics

## 6. Configuration and storage

Use XDG locations:
- Config: `$XDG_CONFIG_HOME/aetherwall/`
- Data: `$XDG_DATA_HOME/aetherwall/`
- Cache: `$XDG_CACHE_HOME/aetherwall/`
- State: `$XDG_STATE_HOME/aetherwall/`

Use sensible XDG defaults when environment variables are absent.

## 7. Rendering decision

The hybrid architecture is intentional. A single custom renderer would add unnecessary complexity for video and browser content. Each runtime should use a mature engine while sharing lifecycle, scheduling, library, display and resource-policy contracts.
