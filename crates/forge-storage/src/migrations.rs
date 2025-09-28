//! Database migrations

use crate::{DbPool, Result};
use tracing::info;

/// Run database migrations
pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    info!("Running database migrations");
    
    // For now, we'll use embedded migrations
    // In a real implementation, these would be separate SQL files
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            avatar_url TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::Error::Storage(format!("Migration failed: {}", e)))?;
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS repositories (
            id TEXT PRIMARY KEY,
            slug TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            default_branch TEXT NOT NULL DEFAULT 'main',
            visibility TEXT NOT NULL DEFAULT 'private',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::Error::Storage(format!("Migration failed: {}", e)))?;
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS issues (
            id TEXT PRIMARY KEY,
            number INTEGER UNIQUE NOT NULL,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            issue_type TEXT NOT NULL,
            state TEXT NOT NULL DEFAULT 'open',
            author_id TEXT NOT NULL,
            repository_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (author_id) REFERENCES users(id),
            FOREIGN KEY (repository_id) REFERENCES repositories(id)
        );
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::Error::Storage(format!("Migration failed: {}", e)))?;
    
    // Create sequence table for global issue numbering
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sequences (
            name TEXT PRIMARY KEY,
            value INTEGER NOT NULL DEFAULT 0
        );
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::Error::Storage(format!("Migration failed: {}", e)))?;
    
    // Initialize issue sequence
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO sequences (name, value) VALUES ('issue_number', 0);
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::Error::Storage(format!("Migration failed: {}", e)))?;
    
    info!("Database migrations completed successfully");
    Ok(())
}