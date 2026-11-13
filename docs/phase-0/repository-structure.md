# Proposed Repository Structure

This is the target architecture, not a requirement to create every directory immediately. Add source directories when their implementation phase starts.

```text
AetherWall/
├── README.md
├── LICENSE
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── SECURITY.md
├── CHANGELOG.md
├── ROADMAP.md
├── AGENTS.md
├── Cargo.toml                  # Rust workspace, introduced with core implementation
├── CMakeLists.txt              # Qt/QML build integration, introduced with UI implementation
│
├── apps/
│   └── aetherwall-studio/
│       ├── qml/
│       │   ├── shell/
│       │   ├── pages/
│       │   ├── dialogs/
│       │   └── components/
│       ├── assets/
│       └── translations/
│
├── crates/
│   ├── aetherwall-core/
│   ├── aetherwall-daemon/
│   ├── aetherwall-ipc/
│   ├── aetherwall-library/
│   ├── aetherwall-scheduler/
│   ├── aetherwall-display/
│   ├── aetherwall-settings/
│   ├── aetherwall-package/
│   └── aetherwall-provider-api/
│
├── runtimes/
│   ├── image/
│   ├── slideshow/
│   ├── video-mpv/
│   ├── web-qtwebengine/
│   ├── anime-media/
│   ├── scene-3d-wgpu/
│   └── game-html5/
│
├── backends/
│   ├── kde-plasma-wayland/
│   ├── kde-plasma-x11/
│   └── compositor-capabilities/
│
├── providers/
│   ├── anime-images/
│   └── anime-videos/
│
├── formats/
│   └── wallpaper-package/
│       ├── schema/
│       └── examples/
│
├── design/
│   ├── README.md
│   ├── design-system/
│   ├── screens/
│   ├── components/
│   ├── mockups/
│   ├── prototypes/
│   ├── themes/
│   ├── animations/
│   ├── icons/
│   └── ux/
│
├── docs/
│   ├── phase-0/
│   ├── architecture/
│   ├── development/
│   ├── user-guide/
│   └── release/
│
├── packaging/
│   ├── arch/
│   ├── systemd/
│   └── desktop/
│
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── contract/
│   ├── security/
│   └── performance/
│
├── scripts/
└── .github/
    ├── ISSUE_TEMPLATE/
    ├── PULL_REQUEST_TEMPLATE.md
    └── workflows/
```

## Boundary rules

- `apps/` contains the user-facing Studio only; it must not own playback lifecycle state.
- `crates/` contains reusable Rust domain logic and the daemon.
- `runtimes/` contains isolated render/playback adapters.
- `backends/` contains desktop-environment and compositor integration.
- `providers/` contains remote catalog adapters; providers do not control the renderer.
- `formats/` owns versioned package schemas and examples.
- `design/` is the single source of truth for visual design.
- `docs/` is the source of truth for product and engineering decisions.
- `packaging/` contains distribution-specific integration.
- `tests/` holds cross-module tests and test fixtures.

## Legacy v4 handling

The existing v4 implementation is a legacy reference. Do not silently delete, rename, or rewrite its HUD, telemetry, Plasma plugins, or installer during Phase 0. The new product explicitly excludes the system-monitoring HUD from its desktop preview and Studio UI. Any future migration or retirement plan must be recorded separately.
