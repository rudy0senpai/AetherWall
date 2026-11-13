//! # Session management
//!
//! Manages wallpaper sessions: their lifecycle, state, and runtime association.

use serde::{Deserialize, Serialize};

use crate::{Display, Wallpaper, WallpaperType};

/// A wallpaper session: a wallpaper running on a display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session UUID.
    pub id: String,
    /// The wallpaper being displayed.
    pub wallpaper: Wallpaper,
    /// Target display.
    pub display: Display,
    /// Runtime type (e.g. "image", "slideshow").
    pub runtime_type: String,
    /// Current state.
    pub state: SessionState,
    /// Runtime session id (opaque to core).
    pub runtime_session_id: Option<String>,
    /// Last error code.
    pub last_error_code: Option<String>,
    /// Last error message.
    pub last_error_message: Option<String>,
    /// Created timestamp (RFC 3339).
    pub created_at: String,
    /// Updated timestamp (RFC 3339).
    pub updated_at: String,
}

impl Session {
    /// Create a new session.
    pub fn new(
        wallpaper: Wallpaper,
        display: Display,
        runtime_type: &str,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Session {
            id: uuid::Uuid::new_v4().to_string(),
            wallpaper,
            display,
            runtime_type: runtime_type.to_string(),
            state: SessionState::Created,
            runtime_session_id: None,
            last_error_code: None,
            last_error_message: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Mark the session as running.
    pub fn running(&mut self) {
        self.state = SessionState::Running;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Mark the session as paused.
    pub fn paused(&mut self, reason: &str) {
        self.state = SessionState::Paused;
        self.updated_at = chrono::Utc::now().to_rfc3339();
        self.last_error_message = Some(reason.to_string());
    }

    /// Mark the session as failed.
    pub fn failed(&mut self, code: &str, message: &str) {
        self.state = SessionState::Failed;
        self.updated_at = chrono::Utc::now().to_rfc3339();
        self.last_error_code = Some(code.to_string());
        self.last_error_message = Some(message.to_string());
    }

    /// Mark the session as stopped.
    pub fn stopped(&mut self) {
        self.state = SessionState::Stopped;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

/// State transition helper: validates allowed transitions.
pub fn validate_transition(from: SessionState, to: SessionState) -> Result<(), String> {
    use SessionState::*;
    use crate::AetherWallError;

    let allowed = match from {
        Created => matches!(to, Preparing | Stopped),
        Preparing => matches!(to, Running | Stopped),
        Running => matches!(to, Paused | Stopped),
        Paused => matches!(to, Running | Stopped),
        Stopped => matches!(to, Created),
        Failed => matches!(to, Stopped),
    };

    if allowed {
        Ok(())
    } else {
        Err(format!(
            "invalid transition from {:?} to {:?}",
            from, to
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(validate_transition(SessionState::Created, SessionState::Preparing).is_ok());
        assert!(validate_transition(SessionState::Prepared, SessionState::Running).is_ok());
        // Prepare is not a valid initial state for Created
        assert!(validate_transition(SessionState::Created, SessionState::Running).is_err());
    }

    #[test]
    fn test_session_functions() {
        let mut s = Session::new(
            Wallpaper {
                id: "test-uuid".to_string(),
                wallpaper_type: WallpaperType::Image,
                source_kind: SourceKind::Local,
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
            Display::test("d1", 1920, 1080),
            "image",
        );
        s.running();
        assert_eq!(s.state, SessionState::Running);
        s.paused("user request");
        assert_eq!(s.state, SessionState::Paused);
        s.failed("runtime-crash", "mpv exited");
        assert_eq!(s.state, SessionState::Failed);
    }
}
