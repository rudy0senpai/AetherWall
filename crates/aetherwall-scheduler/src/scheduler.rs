
//! # Scheduler
//!
//! Slideshow scheduler: manages slideshow session timing.

use std::time::Duration;

use super::playlist::Playlist;

#[derive(Debug, Clone)]
pub struct SlideshowScheduler {
    pub playlist: Playlist,
    pub interval_ms: u64,
    pub running: bool,
    pub current_index: usize,
}

impl SlideshowScheduler {
    pub fn new(playlist: Playlist) -> Self {
        Self {
            playlist,
            interval_ms: playlist.interval_ms,
            running: false,
            current_index: 0,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn next(&mut self) -> Option<&super::playlist::PlaylistItem> {
        if self.items().is_empty() {
            return None;
        }
        self.current_index = self.playlist.next_index(Some(self.current_index));
        self.items().get(self.current_index)
    }

    pub fn items(&self) -> &[super::playlist::PlaylistItem] {
        &self.playlist.items
    }

    pub fn restart(&mut self) {
        self.current_index = 0;
        self.running = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_next() {
        let playlist = Playlist {
            id: "p1".to_string(),
            name: "Test".to_string(),
            mode: PlaylistMode::Sequential,
            interval_ms: 5000,
            transition_type: "fade".to_string(),
            items: vec![
                PlaylistItem { wallpaper_id: "w1".to_string(), position: 0 },
                PlaylistItem { wallpaper_id: "w2".to_string(), position: 1 },
            ],
            created_at: "2026-11-13T00:00:00Z".to_string(),
            updated_at: "2026-11-13T00:00:00Z".to_string(),
        };
        let mut sched = SlideshowScheduler::new(playlist);
        assert_eq!(sched.next().unwrap().wallpaper_id, "w1");
        sched.next();
        assert_eq!(sched.next().unwrap().wallpaper_id, "w2");
        sched.next();
        assert_eq!(sched.next().unwrap().wallpaper_id, "w1");
    }
}
