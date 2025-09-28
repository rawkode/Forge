//! Search functionality for Forge
//!
//! This crate provides global search using Tantivy for indexing and
//! BM25 relevance scoring.

pub mod index;
pub mod query;
pub mod scoring;

pub use forge_core::{Error, Result};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub index_path: String,
    pub max_results: usize,
    pub refresh_interval_seconds: u64,
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub result_type: SearchResultType,
    pub title: String,
    pub content: String,
    pub score: f32,
    pub repository_id: Option<Uuid>,
    pub url: String,
}

/// Types of searchable content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchResultType {
    Code,
    Issue,
    Review,
    Release,
    Documentation,
    Comment,
}