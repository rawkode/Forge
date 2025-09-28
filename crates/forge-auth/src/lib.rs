//! Authentication and Authorization for Forge
//!
//! This crate handles OIDC authentication and JWT token management.

pub mod oidc;
pub mod jwt;
pub mod middleware;

pub use forge_core::{Error, Result};
pub use jwt::JwtManager;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Claims contained in JWT tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub user_id: Uuid,
    pub exp: i64,
    pub iat: i64,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub oidc_issuer: String,
    pub oidc_client_id: String,
    pub oidc_client_secret: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
}