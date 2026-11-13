//! # Error types
//!
//! Structured error types used across the AetherWall codebase.

use thiserror::Error;

/// Result type alias with AetherWall error.
pub type Result<T> = std::result::Result<T, AetherWallError>;

/// Errors thrown by the AetherWall core and daemon.
#[derive(Debug, Error)]
pub enum AetherWallError {
    /// A runtime adapter failed.
    #[error("runtime failed: {0}")]
    RuntimeFailure(String),

    /// Invalid input provided by the caller.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// A D-Bus operation failed.
    #[error("dbus: {0}")]
    DbusError(#[from] dbus::Error),

    /// SQLite database error.
    #[error("sqlite: {0}")]
    SqliteError(#[from] rusqlite::Error),

    /// File or directory operation failed.
    #[error("filesystem: {0}")]
    IoError(#[from] std::io::Error),

    /// Runtime did not start within the timeout.
    #[error("runtime timeout after {0:?}")]
    Timeout(std::time::Duration),

    /// Unknown display.
    #[error("unknown display: {0}")]
    UnknownDisplay(String),

    /// Wallpaper not found.
    #[error("wallpaper not found: {0}")]
    WallpaperNotFound(String),

    /// Package validation failure.
    #[error("package invalid: {0}")]
    PackageInvalid(String),

    /// Provider error.
    #[error("provider error for {provider}: {message}")]
    ProviderError {
        provider: String,
        message: String,
    },

    /// Unsupported feature in the current environment.
    #[error("unsupported: {0}")]
    Unsupported(String),

    /// Generic fallback.
    #[error("{0}")]
    Other(String),
}

impl AetherWallError {
    /// Convert a string slice into an `Other` error.
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }

    /// Convert a string slice into an `InvalidInput` error.
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// Convert a string slice into a `RuntimeFailure` error.
    pub fn runtime_failure(msg: impl Into<String>) -> Self {
        Self::RuntimeFailure(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let e = AetherWallError::invalid_input("path traversal detected");
        assert!(e.to_string().contains("invalid input"));
    }

    #[test]
    fn test_error_from_io() {
        let e = std::io::Error::new(std::io::ErrorKind::Other, "boom");
        let ae: AetherWallError = e.into();
        assert!(matches!(ae, AetherWallError::IoError(_)));
    }
}
