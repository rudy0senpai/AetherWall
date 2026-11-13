# Data Model and Database Design

SQLite is the local persistence store. This is a logical schema proposal; exact SQL migrations are implementation work and must be versioned.

## Core entities

### wallpapers
- `id` TEXT PRIMARY KEY (UUID)
- `package_id` TEXT NULL
- `title` TEXT NOT NULL
- `description` TEXT
- `wallpaper_type` TEXT NOT NULL
- `source_kind` TEXT NOT NULL (`local`, `provider`, `bundled`)
- `source_id` TEXT NULL
- `author_name` TEXT NULL
- `license_id` TEXT NULL
- `manifest_version` TEXT NULL
- `content_path` TEXT NOT NULL
- `thumbnail_path` TEXT NULL
- `preview_path` TEXT NULL
- `width` INTEGER NULL
- `height` INTEGER NULL
- `duration_ms` INTEGER NULL
- `file_size_bytes` INTEGER NULL
- `mime_type` TEXT NULL
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL
- `last_used_at` TEXT NULL
- `is_favorite` INTEGER NOT NULL DEFAULT 0
- `availability_state` TEXT NOT NULL

### displays
- `id` TEXT PRIMARY KEY
- `backend_display_key` TEXT NOT NULL
- `name` TEXT
- `manufacturer` TEXT
- `model` TEXT
- `serial_hash` TEXT NULL
- `logical_x` INTEGER
- `logical_y` INTEGER
- `logical_width` INTEGER
- `logical_height` INTEGER
- `scale_factor` REAL
- `refresh_rate_millihz` INTEGER NULL
- `is_primary` INTEGER NOT NULL DEFAULT 0
- `last_seen_at` TEXT NOT NULL

Do not persist raw serial numbers unless a documented feature requires them. Prefer a non-reversible local identifier.

### wallpaper_sessions
- `id` TEXT PRIMARY KEY
- `wallpaper_id` TEXT NOT NULL REFERENCES wallpapers(id)
- `display_id` TEXT NULL REFERENCES displays(id)
- `runtime_type` TEXT NOT NULL
- `state` TEXT NOT NULL
- `settings_json` TEXT NOT NULL
- `started_at` TEXT NULL
- `updated_at` TEXT NOT NULL
- `last_error_code` TEXT NULL
- `last_error_message` TEXT NULL

### playlists
- `id` TEXT PRIMARY KEY
- `name` TEXT NOT NULL
- `mode` TEXT NOT NULL
- `interval_ms` INTEGER NOT NULL
- `transition_type` TEXT NOT NULL
- `transition_duration_ms` INTEGER NOT NULL
- `shuffle_seed` INTEGER NULL
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL

### playlist_items
- `playlist_id` TEXT NOT NULL REFERENCES playlists(id) ON DELETE CASCADE
- `wallpaper_id` TEXT NOT NULL REFERENCES wallpapers(id)
- `position` INTEGER NOT NULL
- PRIMARY KEY (`playlist_id`, `wallpaper_id`)

### providers
- `id` TEXT PRIMARY KEY
- `provider_key` TEXT UNIQUE NOT NULL
- `display_name` TEXT NOT NULL
- `enabled` INTEGER NOT NULL DEFAULT 0
- `settings_json` TEXT NOT NULL
- `last_sync_at` TEXT NULL
- `last_error` TEXT NULL

### tags
- `id` TEXT PRIMARY KEY
- `name` TEXT UNIQUE NOT NULL

### wallpaper_tags
- `wallpaper_id` TEXT NOT NULL REFERENCES wallpapers(id) ON DELETE CASCADE
- `tag_id` TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE
- PRIMARY KEY (`wallpaper_id`, `tag_id`)

### downloads
- `id` TEXT PRIMARY KEY
- `provider_id` TEXT NULL REFERENCES providers(id)
- `remote_item_id` TEXT NULL
- `target_path` TEXT NOT NULL
- `state` TEXT NOT NULL
- `bytes_received` INTEGER NOT NULL DEFAULT 0
- `bytes_total` INTEGER NULL
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL
- `error_message` TEXT NULL

### settings
- `key` TEXT PRIMARY KEY
- `value_json` TEXT NOT NULL
- `updated_at` TEXT NOT NULL

## Data integrity rules

- Foreign keys enabled.
- Use transactions for imports and migrations.
- Store paths as normalized local paths; do not assume one fixed home directory.
- Index `wallpapers.wallpaper_type`, `wallpapers.last_used_at`, `wallpapers.is_favorite`, provider keys and tag relationships.
- Use UTC RFC 3339 timestamps.
- Avoid storing secrets in SQLite plaintext. Use the desktop secret service/keyring if provider authentication is ever required.
- Never store remote media as installed until download and integrity checks complete.
- Preserve provider attribution and license metadata when supplied.
- Database migrations must be reversible where practical and always have a tested backup/restore path.
