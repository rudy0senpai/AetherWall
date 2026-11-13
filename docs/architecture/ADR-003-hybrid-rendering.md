# ADR-003 — Hybrid rendering

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

AetherWall supports static images, slideshows, videos, web content, 3D scenes,
and interactive games. Each content type has very different rendering and
decoding requirements.

## Decision

Use specialized rendering engines rather than one universal renderer:

| Content type | Engine |
|---|---|
| Image / slideshow | Native image pipeline and scheduler |
| Video | libmpv |
| Web wallpaper | Qt WebEngine |
| 3D wallpaper | wgpu + glTF/GLB |
| Wallpaper games | Constrained HTML5 runtime |

## Reasons

- Avoid reimplementing mature media/browser stacks.
- Each engine is already optimized for its domain.
- A single custom renderer would add complexity for video and browser content.
- Shared lifecycle, scheduling, library, display, and resource-policy contracts
  are maintained across engines.

## Consequences

- Each runtime must implement the runtime lifecycle contract:
  `validate`, `prepare`, `start`, `pause`, `resume`, `update_settings`,
  `stop`, `health`, `dispose`.
- Runtime-specific data must not leak into the shared catalog model.
- Crash isolation is required: runtime crash must not crash Studio or the
  daemon.
