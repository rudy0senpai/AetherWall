//! # Daemon errors
//!
//! Errors specific to the daemon.

use thiserror::Error;

use crate::AetherWallError;

#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("config error: {0}")]
    Config(String),

    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("runtime registry error: {0}")]
    RuntimeRegistry(String),

    #[error("session error: {0}")]
    Session(String),

    #[error("external: {0}")]
    Other(String),
}

impl From<DaemonError> for AetherWallError {
    fn from(e: DaemonError) -> Self {
        match e {
            DaemonError::Config(s) => AetherWallError::other(format!("config: {{}}", s)),
            DaemonError::Database(e) => e.into(),
            DaemonError::RuntimeRegistry(s) => AetherWallError::RuntimeFailure(s),
            DaemonError::Session(s) => AetherWallError::other(s),
            DaemonError::Other(s) => AetherWallError::Other(s),
        }
    }
}
