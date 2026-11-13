//! # Storage
//!
//! SQLite data access for the daemon.

use std::path::Path;

use super::error::DaemonError;
use crate::session::Session;

pub struct Storage {
    pub conn: rusqlite::Connection,
}

impl Storage {
    pub fn open(path: &Path) -> Result<Self, DaemonError> {
        let conn = rusqlite::Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;",
        )?;
        let s = Self { conn };
        s.migrate().map_err(DaemonError::Database)?;
        Ok(s)
    }

    fn migrate(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute_batch(
            r#"CREATE TABLE IF NOT EXISTS wallpapers (
                id TEXT PRIMARY KEY,
                wallpaper_type TEXT NOT NULL,
                source_kind TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                content_path TEXT NOT NULL,
                thumbnail_path TEXT,
                preview_path TEXT,
                width INTEGER,
                height INTEGER,
                duration_ms INTEGER,
                file_size INTEGER,
                mime_type TEXT,
                author TEXT,
                license_id TEXT,
                is_favorite INTEGER NOT NULL DEFAULT 0,
                display_id TEXT,
                last_used_at TEXT
            );
            CREATE TABLE IF NOT EXISTS displays (
                id TEXT PRIMARY KEY,
                backend_key TEXT NOT NULL,
                name TEXT,
                width INTEGER NOT NULL,
                height INTEGER NOT NULL,
                scale_factor REAL NOT NULL,
                refresh_rate_mhz INTEGER,
                is_primary INTEGER NOT NULL DEFAULT 0,
                position_x INTEGER NOT NULL,
                position_y INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                wallpaper_id TEXT NOT NULL,
                display_id TEXT NOT NULL,
                runtime_type TEXT NOT NULL,
                state TEXT NOT NULL,
                runtime_session_id TEXT,
                last_error_code TEXT,
                last_error_message TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value_json TEXT NOT NULL,
                updated_at TEXT NOT NULL
            ); "#,
        )?;
        Ok(())
    }

    pub fn save_session(&self, session: &Session) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            r#"INSERT INTO sessions
               (id, wallpaper_id, display_id, runtime_type, state, runtime_session_id,
                last_error_code, last_error_message, created_at, updated_at)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"#,
            (
                &session.id,
                &session.wallpaper.id,
                &session.display.id,
                &session.runtime_type,
                &match session.state {
                    crate::session::SessionState::Created => "created",
                    crate::session::SessionState::Preparing => "preparing",
                    crate::session::SessionState::Running => "running",
                    crate::session::SessionState::Paused => "paused",
                    crate::session::SessionState::Stopped => "stopped",
                    crate::session::SessionState::Failed => "failed",
                },
                &session.runtime_session_id,
                &session.last_error_code,
                &session.last_error_message,
                &session.created_at,
                &session.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn load_session(&self, id: &str) -> Result<Option<Session>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, wallpaper_id, display_id, runtime_type, state, runtime_session_id,
                      last_error_code, last_error_message, created_at, updated_at
               FROM sessions WHERE id = ?1"#,
        )?;
        let mut rows = stmt.query([id])?;
        if let Some(row) = rows.next()? {
            let session = Session {
                id: row.get(0)?,
                wallpaper: crate::models::Wallpaper {
                    id: row.get(1)?,
                    wallpaper_type: crate::models::WallpaperType::Image,
                    source_kind: crate::models::SourceKind::Local,
                    title: row.get(2)?,
                    description: None,
                    content_path: row.get(3)?,
                    thumbnail_path: None,
                    preview_path: None,
                    width: None,
                    height: None,
                    duration_ms: None,
                    file_size: None,
                    mime_type: None,
                    author: None,
                    license_id: None,
                    is_favorite: false,
                    display_id: None,
                    last_used_at: None,
                },
                display: crate::display::Display {
                    id: row.get(4)?,
                    backend_key: row.get(5)?,
                    name: None,
                    width_mm: None,
                    height_mm: None,
                    logical_width: 1920,
                    logical_height: 1080,
                    scale_factor: 1.0,
                    refresh_rate_mhz: Some(60000),
                    is_primary: false,
                    position_x: 0,
                    position_y: 0,
                },
                runtime_type: row.get(6)?,
                state: enum_from_str(row.get::<_, String>(7)?),
                runtime_session_id: row.get(8)?,
                last_error_code: row.get(9)?,
                last_error_message: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            };
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }
}

fn enum_from_str<T: std::str::FromStr>(s: String) -> Result<T, String> {
    s.parse::<T>().map_err(|_| format!("invalid enum value: {s}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_storage_create_and_load() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("test.db");
        let storage = Storage::open(&path).unwrap();
        let session = crate::session::Session::new(
            crate::models::Wallpaper {
                id: "test-uuid".to_string(),
                wallpaper_type: crate::models::WallpaperType::Image,
                source_kind: crate::models::SourceKind::Local,
                title: "Test".to_string(),
                description: None,
                content_path: "/tmp/test.png".to_string(),
                thumbnail_path: None,
                preview_path: None,
                width: None,
                height: None,
                duration_ms: None,
                file_size: None,
                mime_type: None,
                author: None,
                license_id: None,
                is_favorite: false,
                display_id: None,
                last_used_at: None,
            },
            crate::display::Display::test("d1", 1920, 1080),
            "image",
        );
        storage.save_session(&session).unwrap();
        let loaded = storage.load_session("test-uuid").unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.wallpaper.id, "test-uuid");
        assert_eq!(loaded.runtime_type, "image");
    }
}
