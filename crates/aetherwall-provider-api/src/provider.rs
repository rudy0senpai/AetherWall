//! # Provider
//!
//! Generic provider adapter interface.

use serde::{Deserialize, Serialize};

use super::search::ProviderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub key: String,
    pub display_name: String,
    pub enabled: bool,
    pub settings: serde_json::Value,
}

pub trait Provider: Send + Sync {
    fn key(&self) -> &str;
    fn display_name(&self) -> &str;
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    async fn search(&self, query: &str, page: u32) -> Result<Vec<ProviderResult>, Box<dyn std::error::Error>>;
    async fn download(&self, item_id: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>>;
}
