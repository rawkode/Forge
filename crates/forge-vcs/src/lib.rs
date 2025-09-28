//! Version Control System integration for Forge
//!
//! This crate handles jj (Jujutsu) integration for version control operations.
//! It provides push/pull over HTTPS and repository management.

pub mod jj;
pub mod operations;
pub mod storage;

pub use forge_core::{Error, Result};
use serde::{Deserialize, Serialize};

/// VCS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcsConfig {
    pub repositories_root: String,
    pub jj_binary_path: String,
    pub max_push_size_mb: u64,
    pub max_file_size_mb: u64,
}

/// Repository reference (commit, branch, etc.)
#[derive(Debug, Clone)]
pub struct GitRef {
    pub name: String,
    pub commit_id: String,
    pub ref_type: RefType,
}

#[derive(Debug, Clone)]
pub enum RefType {
    Branch,
    Tag,
    Commit,
}