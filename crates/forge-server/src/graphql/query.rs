//! GraphQL queries

use async_graphql::{Object, Result};

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    /// Get server version and status
    async fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
    
    /// Get server health status
    async fn health(&self) -> Result<HealthStatus> {
        Ok(HealthStatus {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }
}

/// Health status response
#[derive(async_graphql::SimpleObject)]
struct HealthStatus {
    status: String,
    timestamp: String,
}