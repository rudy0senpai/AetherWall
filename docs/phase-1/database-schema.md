# Phase 1 — Database Schema & Migration

**Date:** 2026-11-13
**Branch:** `phase-1/core-implementation`

## 1. Overview

The AetherWall database is a **SQLite** database (per ADR-005). The schema
follows the Phase 0 logical specification (see `docs/phase-0/data-model.md`)
with the following additions:

- `display_assignments` table (per-display wallpaper mapping)
- `import_history` table (import audit trail)
- `runtime_sessions` table (active wallpaper sessions)

## 2. Full schema

```sql
-- Wallpapers: media items in the library
CREATE TABLE IF NOT EXISTS wallpapers (
    id              TEXT PRIMARY KEY,          -- UUID
    wallpaper_type  TEXT NOT NULL,             -- image, slideshow, video, web, anime-image, anime-video, 3d, game
    source_kind     TEXT NOT NULL,             -- local, provider, bundled
    title           TEXT NOT NULL,
    description     TEXT,
    content_path    TEXT NOT NULL,             -- Primary content file path
    thumbnail_path  TEXT,                      -- Generated thumbnail (may be NULL)
    preview_path    TEXT,                      -- Preview image
    width           INTEGER,                   -- pixels
    height          INTEGER,                   -- pixels
    duration_ms     INTEGER,                   -- ms (slideshow/video/web)
    file_size       INTEGER,                   -- bytes
    mime_type       TEXT,                      -- detected MIME type
    author          TEXT,                      -- author/source attribution
    license_id      TEXT,                      -- license identifier
    is_favorite     INTEGER NOT NULL DEFAULT 0,-- boolean flag
    display_id      TEXT,                      -- assigned display (nullable)
    last_used_at    TEXT,                      -- RFC 3339 UTC timestamp
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_wallpapers_type
    ON wallpapers(wallpaper_type, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_wallpapers_favorite
    ON wallpapers(is_favorite);
CREATE INDEX IF NOT EXISTS idx_wallpapers_last_used
    ON wallpapers(last_used_at);

-- Displays: monitor/display topology
CREATE TABLE IF NOT EXISTS displays (
    id                TEXT PRIMARY KEY,        -- UUID
    backend_key       TEXT NOT NULL,           -- DP-1, eDP-1, etc.
    name              TEXT,                    -- human-readable
    width             INTEGER NOT NULL,
    height            INTEGER NOT NULL,
    scale_factor      REAL NOT NULL,           -- e.g. 1.0, 1.25, 1.5
    refresh_rate_mhz  INTEGER,                 -- millihertz
    is_primary        INTEGER NOT NULL DEFAULT 0,
    position_x        INTEGER NOT NULL,
    position_y        INTEGER NOT NULL,
    last_seen_at      TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_displays_primary
    ON displays(is_primary);

-- Display assignments: which wallpaper runs on which display
CREATE TABLE IF NOT EXISTS display_assignments (
    display_id      TEXT NOT NULL,
    wallpaper_id    TEXT NOT NULL,
    wallpaper_type  TEXT NOT NULL,
    fit_mode        TEXT NOT NULL,             -- stretch, fill, fit, center, cover
    PRIMARY KEY (display_id),
    FOREIGN KEY (display_id) REFERENCES displays(id) ON DELETE CASCADE,
    FOREIGN KEY (wallpaper_id) REFERENCES wallpapers(id) ON DELETE SET NULL
);

-- Sessions: active wallpaper sessions
CREATE TABLE IF NOT EXISTS sessions (
    id                TEXT PRIMARY KEY,        -- UUID
    wallpaper_id      TEXT NOT NULL,
    display_id        TEXT,                    -- nullable (global)
    runtime_type      TEXT NOT NULL,           -- image, slideshow, video, etc.
    state             TEXT NOT NULL,           -- created, running, paused, stopped, failed
    runtime_session_id TEXT,                   -- opaque runtime session id
    last_error_code   TEXT,
    last_error_message TEXT,
    created_at        TEXT NOT NULL,
    updated_at        TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_sessions_running
    ON sessions(state) WHERE state = 'running';

-- Playlists (slideshows)
CREATE TABLE IF NOT EXISTS playlists (
    id            TEXT PRIMARY KEY,            -- UUID
    name          TEXT NOT NULL,
    mode          TEXT NOT NULL,               -- sequential, shuffle, random
    interval_ms   INTEGER NOT NULL,
    transition    TEXT NOT NULL,
    created_at    TEXT NOT NULL,
    updated_at    TEXT NOT NULL
);

-- Playlist items
CREATE TABLE IF NOT EXISTS playlist_items (
    playlist_id   TEXT NOT NULL,
    wallpaper_id  TEXT NOT NULL,
    position      INTEGER NOT NULL,
    PRIMARY KEY (playlist_id, wallpaper_id),
    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
    FOREIGN KEY (wallpaper_id) REFERENCES wallpapers(id) ON DELETE CASCADE
);

-- Settings (key-value)
CREATE TABLE IF NOT EXISTS settings (
    key           TEXT PRIMARY KEY,
    value_json    TEXT NOT NULL,
    updated_at    TEXT NOT NULL
);

-- Import history (audit trail)
CREATE TABLE IF NOT EXISTS import_history (
    id            TEXT PRIMARY KEY,            -- UUID
    source_path   TEXT NOT NULL,
    target_path   TEXT NOT NULL,
    wallpaper_id  TEXT,                        -- nullable (may fail)
    state         TEXT NOT NULL,               -- pending, importing, completed, failed
    bytes_received INTEGER NOT NULL DEFAULT 0,
    bytes_total   INTEGER,
    error_message TEXT,
    created_at    TEXT NOT NULL,
    updated_at    TEXT NOT NULL
);

-- Runtime metadata (optional per-runtime fields)
CREATE TABLE IF NOT EXISTS runtime_metadata (
    wallpaper_id    TEXT PRIMARY KEY,
    runtime_type    TEXT NOT NULL,
    metadata        TEXT NOT NULL,             -- JSON blob
    last_health     TEXT,
    FOREIGN KEY (wallpaper_id) REFERENCES wallpapers(id) ON DELETE CASCADE
);
```

## 3. Migrations

### 3.1 Migration strategy

- The database schema is versioned.
- On startup, the daemon checks for the current schema version.
- If the version is older, the daemon applies pending migrations (in order).
- Each migration is a `CREATE TABLE IF NOT EXISTS` / `ALTER TABLE` statement.
- Migrations are idempotent and atomic within a single transaction.

### 3.2 Migration table

```sql
CREATE TABLE IF NOT EXISTS schema_migrations (
    version    INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);
```

### 3.3 Initial version (v1)

The initial schema version is `1`.

```sql
-- Initial schema (v1)
-- See schema above (wallpapers, displays, display_assignments, sessions,
-- playlists, playlist_items, settings, import_history, runtime_metadata)
```

### 3.4 Migration ordering

Migrations are applied in ascending version order. Each migration is wrapped
in a transaction:

```rust
fn migrate(&self) -> Result<(), rusqlite::Error> {
    let current = self.get_current_version()?;
    for version in (self.initial_version + 1)..=current {
        self.apply_migration(version)?;
    }
    Ok(())
}
```

### 3.5 Backup and restore

- The database file is a standard SQLite file.
- The daemon opens the database in WAL mode (`PRAGMA journal_mode = WAL`).
- Backups: copy the database file while the daemon is stopped.
- A tested backup/restore path is required for production.

## 4. Database locations (XDG)

| Directory | Purpose |
|---|---|
| `$XDG_DATA_HOME/aetherwall/aetherwall.db` | SQLite database |
| `$XDG_CACHE_HOME/aetherwall/` | Thumbnail cache, temp files |
| `$XDG_CONFIG_HOME/aetherwall/` | Config files |
| `$XDG_STATE_HOME/aetherwall/` | State files (run-time state) |

## 5. SQLite development note

The development environment has SQLite CLI (`/c/msys64/ucrt64/bin/sqlite3`).
Schema verification: `sqlite3 database/file.db ".schema"`.

The Rust crate uses `rusqlite` with the `bundled` feature (builds the SQLite
library from source). No system SQLite development libraries are required.
