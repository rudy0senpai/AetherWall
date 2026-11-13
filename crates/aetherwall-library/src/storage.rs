
//! # Storage layer
//!
//! SQLite persistence for the wallpaper library.

use rusqlite::Connection;
use std::path::Path;

use super::models::{Display, Session, Wallpaper};

pub struct LibraryStorage {
    conn: Connection,
}

impl LibraryStorage {
    pub fn open(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;",
        )?;
        let s = Self { conn };
        s.migrate()?;
        Ok(s)
    }

    fn migrate(&self) -> Result<(), Box<dyn std::error::Error>> {
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
                display_id TEXT,
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

    pub fn save_wallpaper(&self, w: &Wallpaper) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.execute(
            r#"INSERT OR REPLACE INTO wallpapers
               (id, wallpaper_type, source_kind, title, description, content_path,
                thumbnail_path, preview_path, width, height, duration_ms, file_size,
                mime_type, author, license_id, is_favorite, display_id, last_used_at)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)"#,
            (
                &w.id,
                &w.wallpaper_type,
                &w.source_kind,
                &w.title,
                &w.description,
                &w.content_path,
                &w.thumbnail_path,
                &w.preview_path,
                &w.width,
                &w.height,
                &w.duration_ms,
                &w.file_size,
                &w.mime_type,
                &w.author,
                &w.license_id,
                &w.is_favorite as i32,
                &w.display_id,
                &w.last_used_at,
            ),
        )?;
        Ok(())
    }

    pub fn get_wallpaper(&self, id: &str) -> Result<Option<Wallpaper>, Box<dyn std::error::Error>> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, wallpaper_type, source_kind, title, description, content_path,
                      thumbnail_path, preview_path, width, height, duration_ms, file_size,
                      mime_type, author, license_id, is_favorite, display_id, last_used_at
               FROM wallpapers WHERE id = ?1"#,
        )?;
        let mut rows = stmt.query([id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Wallpaper {
                id: row.get(0)?,
                wallpaper_type: row.get(1)?,
                source_kind: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                content_path: row.get(5)?,
                thumbnail_path: row.get(6)?,
                preview_path: row.get(7)?,
                width: row.get(8)?,
                height: row.get(9)?,
                duration_ms: row.get(10)?,
                file_size: row.get(11)?,
                mime_type: row.get(12)?,
                author: row.get(13)?,
                license_id: row.get(14)?,
                is_favorite: row.get::<_, i32>(15)? != 0,
                display_id: row.get(16)?,
                last_used_at: row.get(17)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_wallpapers(&self, wallpaper_type: Option<&str>) -> Result<Vec<Wallpaper>, Box<dyn std::error::Error>> {
        let mut query = "SELECT id, wallpaper_type, source_kind, title, description, content_path,
                                 thumbnail_path, preview_path, width, height, duration_ms, file_size,
                                 mime_type, author, license_id, is_favorite, display_id, last_used_at
                         FROM wallpapers";
        let mut args = Vec::new();
        if let Some(t) = wallpaper_type {
            query.push_str(" WHERE wallpaper_type = ?1");
            args.push(t.to_string());
        }
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map(args.as_slice(), |row| {
            Ok(Wallpaper {
                id: row.get(0)?,
                wallpaper_type: row.get(1)?,
                source_kind: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                content_path: row.get(5)?,
                thumbnail_path: row.get(6)?,
                preview_path: row.get(7)?,
                width: row.get(8)?,
                height: row.get(9)?,
                duration_ms: row.get(10)?,
                file_size: row.get(11)?,
                mime_type: row.get(12)?,
                author: row.get(13)?,
                license_id: row.get(14)?,
                is_favorite: row.get::<_, i32>(15)? != 0,
                display_id: row.get(16)?,
                last_used_at: row.get(17)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    pub fn save_display(&self, d: &Display) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.execute(
            r#"INSERT OR REPLACE INTO displays
               (id, backend_key, name, width, height, scale_factor, refresh_rate_mhz,
                is_primary, position_x, position_y)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"#,
            (
                &d.id, &d.backend_key, &d.name, &d.width, &d.height, &d.scale_factor,
                &d.refresh_rate_mhz, &d.is_primary as i32, &d.position_x, &d.position_y,
            ),
        )?;
        Ok(())
    }

    pub fn list_displays(&self) -> Result<Vec<Display>, Box<dyn std::error::Error>> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, backend_key, name, width, height, scale_factor, refresh_rate_mhz,
                      is_primary, position_x, position_y FROM displays"#,
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Display {
                id: row.get(0)?,
                backend_key: row.get(1)?,
                name: row.get(2)?,
                width: row.get(3)?,
                height: row.get(4)?,
                scale_factor: row.get(5)?,
                refresh_rate_mhz: row.get(6)?,
                is_primary: row.get::<_, i32>(7)? != 0,
                position_x: row.get(8)?,
                position_y: row.get(9)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    pub fn save_session(&self, s: &Session) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.execute(
            r#"INSERT OR REPLACE INTO sessions
               (id, wallpaper_id, display_id, runtime_type, state, runtime_session_id,
                last_error_code, last_error_message, created_at, updated_at)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"#,
            (
                &s.id, &s.wallpaper_id, &s.display_id, &s.runtime_type, &s.state,
                &s.runtime_session_id, &s.last_error_code, &s.last_error_message,
                &s.created_at, &s.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn list_sessions(&self) -> Result<Vec<Session>, Box<dyn std::error::Error>> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, wallpaper_id, display_id, runtime_type, state, runtime_session_id,
                      last_error_code, last_error_message, created_at, updated_at FROM sessions"#,
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Session {
                id: row.get(0)?,
                wallpaper_id: row.get(1)?,
                display_id: row.get(2)?,
                runtime_type: row.get(3)?,
                state: row.get(4)?,
                runtime_session_id: row.get(5)?,
                last_error_code: row.get(6)?,
                last_error_message: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}
