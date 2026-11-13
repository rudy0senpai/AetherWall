# Phase 0 — Foundation, Product Definition and Design

**Status:** In progress  
**Phase type:** Design → Specification → Prototype → Validation  
**Production implementation:** Out of scope

## Objective

Establish an implementation-ready product definition, target repository layout, architecture boundaries, data contracts, UI/UX system, security model, development conventions, and test strategy before production code begins.

## Fixed decisions

| Area | Decision |
|---|---|
| Product | AetherWall |
| First-class desktop | KDE Plasma 6 |
| Display protocol priority | Wayland first; X11 compatibility where feasible |
| UI | Qt 6 / QML |
| Core | Rust |
| Rendering | Hybrid |
| Video | libmpv |
| Web | Qt WebEngine |
| 3D | wgpu with glTF/GLB |
| Wallpaper games | Sandboxed HTML5 runtime first |
| Persistence | SQLite |
| IPC | D-Bus, with versioned message contracts |
| Initial scope | All eight wallpaper categories, phased delivery |

## Product UI baseline

The application is a two-part studio:

1. **Desktop preview at the top:** a live representation of the selected display and its currently applied wallpaper. It is not a telemetry dashboard.
2. **AetherWall Studio below:** the actual application, with navigation, Library, Workshop, category-specific tools, preview/customization, and settings.

The desktop preview must not contain the former v4 CPU/RAM/battery HUD, graphs, system widgets, or monitoring dock.

## Eight wallpaper categories

1. Pictures
2. Slideshow
3. Videos
4. HTML/CSS/JavaScript web wallpapers
5. Anime pictures from permitted online providers
6. Anime videos from permitted online providers
7. 3D wallpapers
8. Small interactive wallpaper games

## Phase 0 work packages

- P0.1 Repository and documentation baseline
- P0.2 Product requirements and non-functional requirements
- P0.3 Architecture and runtime boundaries
- P0.4 UI/UX and design system
- P0.5 Data model and wallpaper package contract
- P0.6 Security, privacy and provider policy
- P0.7 Development workflow and coding-agent rules
- P0.8 Test strategy and acceptance criteria
- P0.9 Static clickable prototype or high-fidelity screen prototype
- P0.10 Phase 0 review and sign-off

## Exit criteria

Phase 0 is complete only when:

- All required documentation is reviewed and internally consistent.
- Every page and major interaction has a screen specification.
- The desktop preview/application boundary is explicit.
- Runtime interfaces and failure boundaries are documented.
- The package manifest and database entities are specified.
- Security requirements exist for web content, games, imported files and online providers.
- Prototype screens cover the main shell and all eight categories.
- Open questions and unsupported platform capabilities are recorded.
- No production engine code has been introduced under the guise of Phase 0.
