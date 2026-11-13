//! # Library validation
//!
//! Input validation for the wallpaper library.

pub struct LibraryValidator;

impl LibraryValidator {
    /// Validate a file path for import safety.
    pub fn validate_import_path(path: &str) -> Result<(), String> {
        if path.contains("..") {
            return Err("path traversal detected".to_string());
        }
        if path.starts_with('/') || path.contains('\') {
            return Err("absolute or drive-prefixed path not allowed".to_string());
        }
        Ok(())
    }

    /// Validate that a file is a supported image format.
    pub fn validate_image_format(path: &str) -> Result<(), String> {
        let ext = path
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();
        let supported = ["png", "jpg", "jpeg", "webp", "bmp"];
        if supported.contains(&ext.as_str()) {
            Ok(())
        } else {
            Err(format!("unsupported image format: {{}}", ext))
        }
    }
}
