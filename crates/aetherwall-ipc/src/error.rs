//! # IPC errors
//!
//! D-Bus communication and contract errors.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IpcError {
    #[error("dbus: {0}")]
    Dbus(#[from] dbus::Error),

    #[error("service unavailable")]
    ServiceUnavailable,

    #[error("timeout after {{0:?}}")]
    Timeout(std::time::Duration),

    #[error("version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: String, actual: String },

    #[error("invalid message: {0}")]
    InvalidMessage(String),

    #[error("method {0} not found")]
    MethodNotFound(String),

    #[error("error: {0}")]
    Other(String),
}

impl IpcError {
    pub fn service_unavailable() -> Self {
        Self::ServiceUnavailable
    }
}

impl std::convert::From<String> for IpcError {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}
