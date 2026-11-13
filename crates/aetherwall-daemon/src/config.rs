//! # Daemon configuration
//!
//! Loads and persists daemon configuration.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Daemon configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// Path to the SQLite database.
    pub database_path: PathBuf,
    /// Path to the D-Bus service file.
    pub service_file: Option<PathBuf>,
    /// D-Bus service name.
    pub dbus_name: String,
    /// D-Bus object path.
    pub dbus_object_path: String,
    /// Log level (trace, debug, info, warn, error).
    pub log_level: String,
    /// Application data directory (XDG).
    pub data_home: PathBuf,
    /// Application config directory (XDG).
    pub config_home: PathBuf,
    /// Application cache directory (XDG).
    pub cache_home: PathBuf,
    /// Application state directory (XDG).
    pub state_home: PathBuf,
    /// Runtime timeout for wallpaper start (milliseconds).
    pub runtime_start_timeout_ms: u64,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        let config_home = std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                std::env::var("HOME")
                    .map(|h| PathBuf::from(h).join(".config"))
                    .unwrap_or_else(|_| PathBuf::from("."))
            });
        let data_home = std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                std::env::var("HOME")
                    .map(|h| PathBuf::from(h).join(".local/share"))
                    .unwrap_or_else(|_| PathBuf::from("."))
            });
        let cache_home = std::env::var("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                std::env::var("HOME")
                    .map(|h| PathBuf::from(h).join(".cache"))
                    .unwrap_or_else(|_| PathBuf::from("."))
            });
        let state_home = std::env::var("XDG_STATE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                std::env::var("HOME")
                    .map(|h| PathBuf::from(h).join(".local/state"))
                    .unwrap_or_else(|_| PathBuf::from("."))
            });

        DaemonConfig {
            database_path: data_home.join("aetherwall").join("aetherwall.db"),
            service_file: None,
            dbus_name: "org.aetherwall.daemon.1".to_string(),
            dbus_object_path: "/org/aetherwall/daemon/1".to_string(),
            log_level: "info".to_string(),
            data_home,
            config_home,
            cache_home,
            state_home,
            runtime_start_timeout_ms: 10000,
        }
    }
}

impl DaemonConfig {
    /// Load configuration from a file if present, otherwise use defaults.
    pub fn load(path: Option<PathBuf>) -> Self {
        if let Some(p) = path {
            if p.exists() {
                match std::fs::read_to_string(&p) {
                    Ok(s) => {
                        if let Ok(c) = toml::from_str(&s) {
                            return c;
                        }
                    }
                    Err(e) => {
                        eprintln!("warning: failed to read config {}: {{}}", p.display(), e);
                    }
                }
            }
        }
        Self::default()
    }

    /// Save configuration to a file.
    pub fn save(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        let s = toml::to_string_pretty(self).unwrap();
        std::fs::write(path, s)
    }
}
