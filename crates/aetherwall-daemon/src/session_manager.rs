//! # Session manager
//!
//! Manages wallpaper sessions: creation, state tracking, persistence.

use std::collections::HashMap;

use super::storage::Storage;
use core::session::Session;

pub struct SessionManager {
    storage: Storage,
    sessions: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
            sessions: HashMap::new(),
        }
    }

    pub fn create(
        &mut self,
        wallpaper: crate::models::Wallpaper,
        display: crate::display::Display,
        runtime_type: &str,
    ) -> Session {
        let session = core::session::Session::new(wallpaper, display, runtime_type);
        self.sessions.insert(session.id.clone(), session.clone());
        self.storage.save_session(&session).ok();
        session
    }

    pub fn get(&self, id: &str) -> Option<&Session> {
        self.sessions.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    }

    pub fn list_active(&self) -> Vec<&Session> {
        self.sessions
            .values()
            .filter(|s| s.state == core::session::SessionState::Running)
            .collect()
    }

    pub fn remove(&mut self, id: &str) -> Option<Session> {
        self.sessions.remove(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session_manager::SessionManager;

    #[test]
    fn test_session_create_get_remove() {
        let storage = Storage::in_memory();
        let mut mgr = SessionManager::new(storage);
        let session = mgr.create(
            crate::models::Wallpaper {
                id: "uuid-1".to_string(),
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
        assert_eq!(session.state, core::session::SessionState::Created);
        assert!(mgr.get("uuid-1").is_some());
        mgr.remove("uuid-1");
        assert!(mgr.get("uuid-1").is_none());
    }
}
