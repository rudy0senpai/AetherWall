# ADR-004 — KDE Plasma 6 first

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

AetherWall targets a Linux desktop and must apply wallpapers to the users'
displays. The question is which desktop/compositor integration to prioritize.

## Decision

KDE Plasma 6 is the first-class desktop target; **Wayland is prioritized** and
X11 is a compatibility path.

## Reasons

- A defined integration target reduces initial platform ambiguity.
- Plasma provides the extension/widget APIs needed to apply wallpapers reliably.
- Wayland is the default for new KDE installs.

## Consequences

- Do not promise generic Wayland support without validating compositor APIs.
- Generic Wayland does **not** permit arbitrary desktop-background windows.
- X11 is a separate compatibility backend.
- The backend capability detection must identify supported wallpaper surfaces,
  monitor geometry, focus/fullscreen signals, and lifecycle events.
- Unsupported compositor features must be reported honestly.
