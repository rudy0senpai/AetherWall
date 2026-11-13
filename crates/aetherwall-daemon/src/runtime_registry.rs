
//! # Runtime registry
//!
//! Registers and iterates wallpaper runtime adapters.

use std::collections::HashMap;

use crate::runtime_registry::WallpaperRuntime;

pub use crate::runtime_registry::WallpaperRuntime;
use crate::{Display, Window, Wallpaper, WallpaperType};

/// Trait implemented by all wallpaper runtime adapters.
pub trait WallpaperRuntime: Send + Sync {
    fn runtime_type(&self) -> &'static str;
    fn supported_types(&self) -> &'static [WallpaperType];
    async fn validate(&self, wallpaper: &Wallpaper) -> Result<(), Box<AetherWallError>>;
    async fn prepare(
        &self,
        wallpaper: &Wallpaper,
        display: &Display,
        settings: &Window,
    ) -> Result<(), Box<AetherWallError>>;
    async fn start(&self, session_id: &str) -> Result<(), Box<AetherWallError>>;
    async fn pause(&self, session_id: &str, reason: &str) -> Result<(), Box<AetherWallError>>;
    async fn resume(&self, session_id: &str) -> Result<(), Box<AetherWallError>>;
    async fn update_settings(
        &self,
        session_id: &str,
        settings: &Window,
    ) -> Result<(), Box<AetherWallError>>;
    async fn stop(&self, session_id: &str) -> Result<(), Box<AetherWallError>>;
    async fn health(&self, session_id: &str) -> Result<super::runtime::HealthStatus, Box<AetherWallError>>;
    async fn dispose(&self, session_id: &str) -> Result<(), Box<AetherWallError>>;
}

pub struct RuntimeRegistry {
    runtimes: Vec<Box<dyn WallpaperRuntime>>,
}

impl RuntimeRegistry {
    pub fn new() -> Self {
        Self { runtimes: Vec::new() }
    }

    pub fn register(&mut self, runtime: Box<dyn WallpaperRuntime>) {
        self.runtimes.push(runtime);
    }

    pub fn get(&self, runtime_type: &str) -> Option<&dyn WallpaperRuntime> {
        self.runtimes
            .iter()
            .find(|r| r.runtime_type() == runtime_type)
            .map(|r| r.as_ref())
    }

    pub fn supported_types(&self) -> Vec<WallpaperType> {
        let mut types = Vec::new();
        for r in &self.runtimes {
            for t in r.supported_types() {
                if !types.contains(t) {
                    types.push(*t);
                }
            }
        }
        types
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_registry::WallpaperRuntime;

    struct MockRuntime {
        r#type: String,
        supported: Vec<WallpaperType>,
        started: bool,
    }

    impl MockRuntime {
        fn new(r#type: &str, supported: Vec<WallpaperType>) -> Self {
            Self {
                r#type: r#type.to_string(),
                supported,
                started: false,
            }
        }
    }

    #[async_trait::async_trait]
    impl WallpaperRuntime for MockRuntime {
        fn runtime_type(&self) -> &'static str {
            self.r#type.as_str()
        }

        fn supported_types(&self) -> &'static [WallpaperType] {
            self.supported.as_slice()
        }

        async fn validate(
            &self,
            _: &super::super::models::Wallpaper,
        ) -> Result<(), Box<AetherWallError>> {
            Ok(())
        }

        async fn prepare(
            &self,
            _: &super::super::models::Wallpaper,
            _: &super::super::display::Display,
            _: &super::super::models::Window,
        ) -> Result<(), Box<AetherWallError>> {
            Ok(())
        }

        async fn start(&self, _: &str) -> Result<(), Box<AetherWallError>> {
            self.started = true;
            Ok(())
        }

        async fn pause(
            &self,
            _: &str,
            _: &str,
        ) -> Result<(), Box<AetherWallError>> {
            Ok(())
        }

        async fn resume(&self, _: &str) -> Result<(), Box<AetherWallError>> {
            Ok(())
        }

        async fn update_settings(
            &self,
            _: &str,
            _: &super::super::models::Window,
        ) -> Result<(), Box<AetherWallError>> {
            Ok(())
        }

        async fn stop(&self, _: &str) -> Result<(), Box<AetherWallError>> {
            Ok(())
        }

        async fn health(
            &self,
            _: &str,
        ) -> Result<super::super::runtime::HealthStatus, Box<AetherWallError>> {
            Ok(super::super::runtime::HealthStatus::Healthy)
        }

        async fn dispose(&self, _: &str) -> Result<(), Box<AetherWallError>> {
            self.started = false;
            Ok(())
        }
    }

    #[test]
    fn test_registry_lifecycle() {
        let mut reg = RuntimeRegistry::new();
        reg.register(Box::new(MockRuntime::new("image", vec![WallpaperType::Image])));

        let entry = reg.get("image").expect("runtime not found");
        assert_eq!(entry.runtime_type(), "image");
        assert!(reg.supported_types().contains(&WallpaperType::Image));
    }
}
