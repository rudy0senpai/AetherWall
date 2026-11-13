# AetherWall

**A Linux-native wallpaper platform for KDE Plasma 6.**

AetherWall combines a wallpaper library, discovery workshop, desktop preview, and
modular playback runtimes for static images, slideshows, videos, web content,
anime media, 3D scenes, and interactive wallpaper games.

## Product UI baseline

The application is a two-part studio:

1. **Desktop preview at the top** — a live representation of the selected display
   and its currently applied wallpaper. It is a preview surface, not a
   telemetry dashboard.
2. **AetherWall Studio below** — the actual application, with navigation, Library,
   Workshop, category-specific tools, preview/customization, and settings.

The desktop preview does **not** contain the former v4 CPU/RAM/battery HUD,
graphs, system widgets, or monitoring dock.

## Current phases

### Phase 0 — Foundation, product definition, and design

In progress. Design → specification → prototype → validation. No production
playback implementation.

- Repository and documentation baseline
- Product requirements and non-functional requirements
- Architecture and runtime boundaries
- UI/UX and design system
- Data model and wallpaper package contract
- Security, privacy and provider policy
- Development workflow and coding-agent rules
- Test strategy and acceptance criteria
- Static clickable prototype / high-fidelity screen prototypes
- Phase 0 review and sign-off

See [docs/phase-0/README.md](docs/phase-0/README.md) for the full plan and
acceptance criteria.

## Technology direction

| Area | Technology |
|---|---|
| UI | Qt 6 / QML |
| Core | Rust (daemon + domain crates) |
| Persistence | SQLite |
| IPC | D-Bus (versioned) |
| Video | libmpv |
| Web wallpaper | Qt WebEngine |
| 3D wallpaper | wgpu + glTF/GLB |
| Wallpaper games | Sandboxed HTML5 runtime |
| Desktop integration | KDE Plasma 6 (Wayland first, X11 where practical) |

## Repository map

See [docs/phase-0/repository-structure.md](docs/phase-0/repository-structure.md)
for the proposed target repository tree. The target tree is a design contract;
directories are added as their implementation phase starts.

## Documentation

All Phase 0 documentation is in [docs/phase-0/](docs/phase-0/):

| Document | Description |
|---|---|
| [README.md](docs/phase-0/README.md) | Phase 0 plan and acceptance criteria |
| [repository-structure.md](docs/phase-0/repository-structure.md) | Target repository tree |
| [product-requirements.md](docs/phase-0/product-requirements.md) | Product vision, requirements, non-functional |
| [architecture.md](docs/phase-0/architecture.md) | High-level architecture and runtime lifecycle |
| [ui-ux-specification.md](docs/phase-0/ui-ux-specification.md) | Product shell, navigation, design tokens |
| [data-model.md](docs/phase-0/data-model.md) | SQLite logical schema |
| [wallpaper-package-specification.md](docs/phase-0/wallpaper-package-specification.md) | `.awall` package format |
| [architecture-decisions.md](docs/phase-0/architecture-decisions.md) | ADRs |
| [testing-and-validation.md](docs/phase-0/testing-and-validation.md) | Test strategy |
| [development-workflow.md](docs/phase-0/development-workflow.md) | Coding-agent workflow |
| [acceptance-checklist.md](docs/phase-0/acceptance-checklist.md) | Phase 0 acceptance checklist |

## Principles

1. The wallpaper remains the desktop's primary visual surface.
2. The application UI and wallpaper rendering process are separate.
3. A crash in a wallpaper runtime must not crash the Studio UI or core service.
4. Every wallpaper type implements a common lifecycle contract.
5. Heavy rendering is paused or reduced when hidden, locked, or covered by
   fullscreen workloads where the compositor permits reliable detection.
6. Online providers are adapters; provider-specific logic must not leak into the
   core domain model.
7. No wallpaper package is trusted merely because it is installed.
8. Features must report unsupported compositor capabilities honestly instead of
   pretending they work.
