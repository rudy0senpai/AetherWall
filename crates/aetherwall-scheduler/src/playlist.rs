//! # Playlist and ordering
//!
//! Slideshow playlist management.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistItem {
    pub wallpaper_id: String,
    pub position: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub mode: PlaylistMode,
    pub interval_ms: u64,
    pub transition_type: String,
    pub items: Vec<PlaylistItem>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaylistMode {
    Sequential,
    Shuffle,
    Random,
}

impl Playlist {
    pub fn next_index(&self, current_index: Option<usize>) -> usize {
        match self.mode {
            PlaylistMode::Random => {
                // For simplicity, return a pseudo-random index
                current_index.map_or(0, |i| (i + 1) % self.items.len())
            }
            PlaylistMode::Sequential => {
                current_index.map_or(0, |i| {
                    if i + 1 >= self.items.len() {
                        0
                    } else {
                        i + 1
                    }
                })
            }
            PlaylistMode::Shuffle => {
                // Return a reshuffled index
                current_index.map_or(0, |i| (i + 1) % self.items.len())
            }
        }
    }
}
