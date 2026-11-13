//! # Event bus
//!
//! Pub/sub event bus for daemon events delivered to the Studio UI.

use serde::{Deserialize, Serialize};
use std::sync::mpsc;

/// Typed events published by the daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonEvent {
    LibraryItemAdded { id: String },
    LibraryItemUpdated { id: String },
    LibraryItemRemoved { id: String },
    ImportProgress {
        path: String,
        bytes_received: u64,
        bytes_total: Option<u64>,
    },
    ImportCompleted { path: String },
    ImportFailed { path: String, error: String },
    SessionStateChanged {
        session_id: String,
        state: core::session::SessionState,
    },
    DisplayTopologyChanged,
    ProviderProgress {
        provider: String,
        bytes_received: u64,
        bytes_total: Option<u64>,
    },
    RuntimeHealthChanged {
        runtime_type: String,
        session_id: String,
        healthy: bool,
    },
    ErrorReported {
        error_code: String,
        error_message: String,
    },
}

/// Thread-local event bus (in-memory).
pub struct EventBus {
    tx: mpsc::Sender<DaemonEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            while let Ok(ev) = rx.recv() {
                eprintln!("[aetherwall] event: {:?}", ev);
            }
        });
        Self { tx }
    }

    pub fn publish(&self, event: DaemonEvent) {
        let _ = self.tx.send(event);
    }

    pub fn subscribe(self) -> mpsc::Receiver<DaemonEvent> {
        self.tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let ev = DaemonEvent::LibraryItemAdded {
            id: "uuid-1234".to_string(),
        };
        let json = serde_json::to_string(&ev).unwrap();
        assert!(json.contains("uuid-1234"));
        let decoded: DaemonEvent = serde_json::from_str(&json).unwrap();
        assert!(matches!(decoded, DaemonEvent::LibraryItemAdded { .. }));
    }
}
