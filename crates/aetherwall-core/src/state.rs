//! # Application state
//!
//! Global application state managed by the daemon.

use serde::{Deserialize, Serialize};

/// Application-level configuration keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigKey {
    Theme,
    Language,
    DefaultFitMode,
    ShowDesktopPreview,
    EnableDiagnostics,
}

impl std::fmt::Display for ConfigKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Theme => write!(f, "theme"),
            Self::Language => write!(f, "language"),
            Self::DefaultFitMode => write!(f, "default_fit_mode"),
            Self::ShowDesktopPreview => write!(f, "show_desktop_preview"),
            Self::EnableDiagnostics => write!(f, "enable_diagnostics"),
        }
    }
}

/// Application configuration stored in SQLite (settings table).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub key: String,
    pub value_json: String,
    pub updated_at: String,
}

/// Connection state of the daemon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DaemonState {
    /// Daemon is running and listening on D-Bus.
    Running,
    /// Daemon is initializing (config loaded, database open).
    Starting,
    /// Daemon is shutting down.
    ShuttingDown,
    /// Daemon is not running (crashed or not started).
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonInfo {
    pub pid: Option<u32>,
    pub state: DaemonState,
    pub uptime_ms: u64,
    pub database_path: Option<String>,
    pub supported_runtimes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_key_display() {
        assert_eq!(ConfigKey::Theme.to_string(), "theme");
        assert_eq!(ConfigKey::Language.to_string(), "language");
    }
}
