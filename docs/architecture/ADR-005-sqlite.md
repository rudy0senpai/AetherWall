# ADR-005 — SQLite local database

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

AetherWall needs a local persistence store for the wallpaper library, playlists,
settings, provider state, and sessions.

## Decision

Use **SQLite** for local persistence.

## Reasons

- Local-first desktop application with no mandatory server dependency.
- Mature, transactional, cross-platform.
- Supports the data model (entities, indexes, migrations) defined in
  `docs/phase-0/data-model.md`.
- Suitable for a per-user, single-user service.

## Consequences

- Foreign keys enabled.
- Transactions used for imports and migrations.
- Migrations must be reversible where practical and always have a tested
  backup/restore path.
- Avoid storing secrets in SQLite plaintext; use the desktop secret
  service/keyring if provider authentication is ever added.
- Do not persist raw serial numbers; prefer a non-reversible local identifier.
