# Architecture Decision Records — Phase 0

## ADR-001: Qt 6 / QML for Studio

**Decision:** Use Qt 6 / QML for the desktop application UI.

**Reason:** Mature Linux desktop toolkit, strong KDE ecosystem fit, reusable components and suitable multimedia/UI integration.

**Consequence:** QML remains presentation-focused; core state and long-running work are owned by the daemon.

## ADR-002: Rust core and daemon

**Decision:** Use Rust for the background service and shared domain logic.

**Reason:** Strong type system, concurrency and lifecycle control for a long-running process.

**Consequence:** Establish stable IPC contracts between Rust services and Qt/QML.

## ADR-003: Hybrid rendering

**Decision:** Use specialized rendering engines rather than one universal renderer.

- Video: libmpv
- Web: Qt WebEngine
- 3D: wgpu + glTF
- Games: constrained HTML5 runtime
- Images/slideshows: native image pipeline and scheduler

**Reason:** Avoid reimplementing mature media/browser stacks while preserving a common lifecycle model.

## ADR-004: KDE Plasma 6 first

**Decision:** KDE Plasma 6 is the first-class desktop target; Wayland is prioritized and X11 is a compatibility path.

**Reason:** A defined integration target reduces initial platform ambiguity.

**Consequence:** Do not promise generic Wayland support without validating compositor APIs and capabilities.

## ADR-005: SQLite local database

**Decision:** SQLite stores library metadata, playlists, settings, provider state and sessions.

**Reason:** Local-first desktop application with no mandatory server dependency.

## ADR-006: Wallpaper preview above Studio

**Decision:** Main application layout places a desktop wallpaper preview at the top and the AetherWall Studio interface below.

**Explicit exclusion:** No CPU/RAM/battery monitoring HUD or system graph overlay is part of this product design.

## ADR-007: Package format is declarative

**Decision:** `.awall` is a ZIP-compatible package with a versioned JSON manifest. Native executable payloads are prohibited.

**Reason:** Portability, validation and reduced supply-chain risk.
