//! Configuration types and utilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Forge configuration from forge.cue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    pub default_visibility: crate::types::Visibility,
    pub allowed_domains: Vec<String>,
    pub artifact_retention: crate::types::RetentionPolicy,
    pub limits: Limits,
}

/// System limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limits {
    pub push_body_limit_mb: u64,
    pub file_size_ceiling_mb: u64,
    pub max_repositories_per_product: u32,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            push_body_limit_mb: 200,
            file_size_ceiling_mb: 100,
            max_repositories_per_product: 50,
        }
    }
}

/// Repository configuration from forge.cue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoConfig {
    pub slug: crate::types::Slug,
    pub default_branch: String,
    pub branch_protections: HashMap<String, BranchProtection>,
    pub labels: Vec<crate::types::Label>,
    pub code_owners: Vec<CodeOwnerRule>,
}

/// Branch protection rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchProtection {
    pub required_approvals: u32,
    pub require_up_to_date: bool,
    pub require_code_owner_approval: bool,
    pub require_signed_commits: bool,
    pub allowed_merge_types: Vec<MergeType>,
}

/// Code owner rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeOwnerRule {
    pub pattern: String, // Glob pattern
    pub owners: Vec<String>, // Users or teams
}

/// Merge types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeType {
    FastForward,
    Merge,
    Squash,
    Rebase,
}

/// Policy grants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyGrant {
    pub resource: String,
    pub target: String,
    pub role: String,
    pub subjects: Vec<String>,
}