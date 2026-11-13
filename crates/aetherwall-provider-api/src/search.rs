//! # Provider search results
//!
//! Normalized search result structure for providers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResult {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub source_kind: String,
    pub source_id: String,
    pub thumbnail_path: Option<String>,
    pub author: Option<String>,
    pub license_id: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub file_size: Option<u64>,
}
