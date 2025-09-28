//! Core domain types for Forge

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A slug identifier - lowercase alphanumeric with dots, underscores, and hyphens
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Slug(String);

impl Slug {
    pub fn new(s: &str) -> Result<Self, crate::Error> {
        if s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '_' || c == '-') {
            Ok(Slug(s.to_string()))
        } else {
            Err(crate::Error::Validation("Invalid slug format".to_string()))
        }
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Distributed identifier for entities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Did(String);

impl Did {
    pub fn new(s: String) -> Self {
        Did(s)
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: Uuid,
    pub slug: Slug,
    pub name: String,
    pub description: Option<String>,
    pub default_branch: String,
    pub visibility: Visibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Product configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub repositories: Vec<Uuid>,
    pub config: ProductConfig,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Product configuration from CUE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductConfig {
    pub docs: DocsConfig,
    pub issues: IssuesConfig,
    pub releases: ReleasesConfig,
}

/// Documentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsConfig {
    pub roots: Vec<String>,
    pub allow_list: Vec<String>,
}

/// Issues configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuesConfig {
    pub types: Vec<IssueType>,
    pub labels: Vec<Label>,
    pub workflows: HashMap<String, IssueWorkflow>,
}

/// Release configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleasesConfig {
    pub dirs: Vec<String>,
    pub validation: ReleaseValidation,
    pub retention: RetentionPolicy,
}

/// Issue type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueType {
    pub name: String,
    pub schema: serde_json::Value, // CUE schema
    pub default_labels: Vec<String>,
}

/// Issue workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueWorkflow {
    pub states: Vec<String>,
    pub transitions: HashMap<String, Vec<String>>,
}

/// Label definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

/// Release validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseValidation {
    pub required_files: Vec<String>,
    pub max_artifact_size: u64,
}

/// Artifact retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub keep_latest: u32,
    pub keep_days: u32,
}

/// Visibility settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Private,
    Public,
}

/// Issue entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: Uuid,
    pub number: u64, // Global sequence
    pub title: String,
    pub body: String,
    pub issue_type: String,
    pub state: String,
    pub labels: Vec<String>,
    pub assignees: Vec<Uuid>,
    pub author: Uuid,
    pub repository_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Review entity (PR-style)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub state: ReviewState,
    pub source_ref: String,
    pub target_ref: String,
    pub repository_id: Uuid,
    pub author: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Review state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewState {
    Open,
    Approved,
    ChangesRequested,
    Merged,
    Closed,
}

/// Release entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub commit_id: String,
    pub status: ReleaseStatus,
    pub artifacts: Vec<ReleaseArtifact>,
    pub repository_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Release status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReleaseStatus {
    Draft,
    Published,
    Archived,
}

/// Release artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseArtifact {
    pub name: String,
    pub content_type: String,
    pub size: u64,
    pub sha256: String,
    pub s3_key: String,
}

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}