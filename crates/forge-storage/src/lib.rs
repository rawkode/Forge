//! Storage layer for Forge
//!
//! This crate provides the database abstraction and migrations for Forge.
//! It uses SQLite/libsql for storage.

pub mod migrations;
pub mod repository;
pub mod models;

pub use forge_core::{Error, Result};
use serde::{Deserialize, Serialize};

use sqlx::SqlitePool;

/// Database connection pool
pub type DbPool = SqlitePool;

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub backup_interval_hours: u64,
}

/// Initialize the database with migrations
pub async fn initialize_database(config: &StorageConfig) -> Result<DbPool> {
    let pool = SqlitePool::connect(&config.database_url)
        .await
        .map_err(|e| Error::Storage(format!("Failed to connect to database: {}", e)))?;
    
    // Run migrations
    migrations::run_migrations(&pool).await?;
    
    Ok(pool)
}