# ADR-001 — Qt 6 / QML for Studio

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

AetherWall needs a desktop application UI that integrates with the KDE Plasma
ecosystem, supports large media previews, and stays responsive during disk,
network, decode, and render operations.

## Decision

Use **Qt 6 / QML** for the desktop application UI.

## Reasons

- Mature Linux desktop toolkit with strong KDE ecosystem fit.
- Reusable components and centralized design tokens.
- Suitable multimedia/UI integration for wallpaper previews and playback.
- QML is presentation-focused, which keeps the UI layer separate from core.

## Consequences

- QML remains presentation-focused; core state and long-running work are owned
  by the daemon.
- UI must never block on disk, network, decode, or render operations.
- A stable D-Bus contract is required between QML and the Rust daemon.
