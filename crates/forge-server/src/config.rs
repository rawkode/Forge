//! Server configuration

use anyhow::Result;
use forge_auth::AuthConfig;
use forge_policy::PolicyConfig;
use forge_search::SearchConfig;
use forge_storage::StorageConfig;
use forge_vcs::VcsConfig;
use serde::{Deserialize, Serialize};

/// Main server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub auth: AuthConfig,
    pub storage: StorageConfig,
    pub vcs: VcsConfig,
    pub search: SearchConfig,
    pub policy: PolicyConfig,
    pub server: HttpConfig,
    pub tls: Option<TlsConfig>,
}

/// HTTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub request_timeout_seconds: u64,
    pub max_request_size_mb: u64,
    pub cors_origins: Vec<String>,
}

/// TLS configuration with ACME support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub acme_directory: String,
    pub acme_email: String,
    pub domains: Vec<String>,
    pub cache_dir: String,
}

impl ServerConfig {
    pub async fn load(path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            auth: AuthConfig {
                oidc_issuer: "https://accounts.google.com".to_string(),
                oidc_client_id: "".to_string(),
                oidc_client_secret: "".to_string(),
                jwt_secret: "change-me-in-production".to_string(),
                jwt_expiry_hours: 24,
            },
            storage: StorageConfig {
                database_url: "sqlite:forge.db".to_string(),
                max_connections: 10,
                backup_interval_hours: 6,
            },
            vcs: VcsConfig {
                repositories_root: "./repositories".to_string(),
                jj_binary_path: "jj".to_string(),
                max_push_size_mb: 200,
                max_file_size_mb: 100,
            },
            search: SearchConfig {
                index_path: "./search_index".to_string(),
                max_results: 100,
                refresh_interval_seconds: 300,
            },
            policy: PolicyConfig {
                policy_store_path: "./policies".to_string(),
                evaluation_timeout_ms: 100,
            },
            server: HttpConfig {
                request_timeout_seconds: 30,
                max_request_size_mb: 100,
                cors_origins: vec!["http://localhost:3000".to_string()],
            },
            tls: None,
        }
    }
}