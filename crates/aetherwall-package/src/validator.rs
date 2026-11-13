//! # Package validator
//!
//! Validates `.awall` package structure and manifest.

use crate::manifest::WallpaperManifest;

pub struct PackageValidator;

impl PackageValidator {
    pub fn validate_manifest(&self, manifest: &WallpaperManifest) -> Result<(), String> {
        if manifest.schema_version != "1.0" {
            return Err(format!("unsupported schema version: {{}}", manifest.schema_version));
        }
        if manifest.id.is_empty() {
            return Err("manifest id is required".to_string());
        }
        if manifest.name.is_empty() {
            return Err("manifest name is required".to_string());
        }
        if manifest.version.is_empty() {
            return Err("manifest version is required".to_string());
        }
        if manifest.r#type.is_empty() {
            return Err("manifest type is required".to_string());
        }
        Ok(())
    }

    pub fn validate_archive_path(&self, path: &str) -> Result<(), String> {
        if path.contains("..") {
            return Err("path traversal detected".to_string());
        }
        if path.starts_with('/') || path.contains('\\') {
            return Err("absolute or drive-prefixed path not allowed".to_string());
        }
        Ok(())
    }
}
