# ADR-002 — Rust core and daemon

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

AetherWall needs a background service that owns wallpaper catalog, sessions,
scheduling, display mapping, settings, runtime supervision, and event delivery.

## Decision

Use **Rust** for the background service (`aetherwall-daemon`) and shared domain
logic (`crates/`).

## Reasons

- Strong type system and concurrency model for a long-running process.
- Lifecycle control for critical subsystems.
- Clear boundary between the Rust core and the QML UI.
- Memory safety is valuable for a daemon that manages many content runtimes.

## Consequences

- Establish stable IPC contracts (D-Bus) between Rust services and Qt/QML.
- Define versioned message schemas before implementation.
- Runtime adapters must be behind traits/interfaces so the daemon remains
  independent of renderer details.
