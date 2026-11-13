# ADR-006 — Wallpaper preview above Studio

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

AetherWall's product UI baseline is a two-part studio: a desktop wallpaper
preview at the top and the AetherWall Studio application below.

## Decision

The main application layout places a **desktop wallpaper preview at the top**
and the **AetherWall Studio interface below**.

## Explicit exclusion

No CPU/RAM/battery monitoring HUD or system graph overlay is part of this
product design.

## Reasons

- The wallpaper remains the desktop's primary visual surface.
- Users expect to see the applied output before interacting with the Studio.
- Separating preview from Studio keeps the UI clean and focused.

## Consequences

- The desktop preview must not contain the former v4 CPU/RAM/battery HUD,
  graphs, system widgets, or monitoring dock.
- Preview state and Studio state are separate, but linked through display
  selection and apply actions.
