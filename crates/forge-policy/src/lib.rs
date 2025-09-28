//! Policy engine for Forge
//!
//! This crate integrates with Cedar for authorization decisions.
//! Policies are compiled from CUE configuration at main HEAD.

pub mod cedar;
pub mod compiler;
pub mod evaluator;

pub use forge_core::{Error, Result};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub policy_store_path: String,
    pub evaluation_timeout_ms: u64,
}

/// Authorization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub principal: Principal,
    pub action: String,
    pub resource: Resource,
    pub context: AuthContext,
}

/// Principal (user or service)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub id: Uuid,
    pub principal_type: PrincipalType,
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrincipalType {
    User,
    Service,
}

/// Resource being accessed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub resource_type: String,
    pub attributes: std::collections::HashMap<String, serde_json::Value>,
}

/// Authorization context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub ip_address: Option<std::net::IpAddr>,
    pub user_agent: Option<String>,
}

/// Authorization decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Decision {
    Allow,
    Deny,
}

/// Authorization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    pub decision: Decision,
    pub reasons: Vec<String>,
}