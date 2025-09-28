//! Database repository layer

use crate::{models::*, DbPool, Result};
use forge_core::types::*;
use sqlx::Row;
use uuid::Uuid;

/// Repository for user operations
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    
    /// Find a user by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let model = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM users WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::Error::Storage(format!("Database error: {}", e)))?;
        
        Ok(model.map(|m| m.into()))
    }
    
    /// Find a user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let model = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM users WHERE email = ?",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::Error::Storage(format!("Database error: {}", e)))?;
        
        Ok(model.map(|m| m.into()))
    }
    
    /// Create a new user
    pub async fn create(&self, user: &User) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (id, email, name, avatar_url, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.id.to_string())
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.avatar_url)
        .bind(user.created_at.to_rfc3339())
        .bind(user.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| crate::Error::Storage(format!("Database error: {}", e)))?;
        
        Ok(())
    }
}

/// Repository for repository operations
pub struct RepoRepository {
    pool: DbPool,
}

impl RepoRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    
    /// List all repositories
    pub async fn list_all(&self) -> Result<Vec<Repository>> {
        let models = sqlx::query_as::<_, RepositoryModel>(
            "SELECT * FROM repositories ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| crate::Error::Storage(format!("Database error: {}", e)))?;
        
        let repositories: Result<Vec<_>> = models
            .into_iter()
            .map(|m| m.try_into())
            .collect();
        
        repositories
    }
    
    /// Get next issue number
    pub async fn next_issue_number(&self) -> Result<u64> {
        let mut tx = self.pool.begin().await
            .map_err(|e| crate::Error::Storage(format!("Transaction error: {}", e)))?;
        
        let current: i64 = sqlx::query("SELECT value FROM sequences WHERE name = 'issue_number'")
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| crate::Error::Storage(format!("Database error: {}", e)))?
            .try_get(0)
            .map_err(|e| crate::Error::Storage(format!("Database error: {}", e)))?;
        
        let next = current + 1;
        
        sqlx::query("UPDATE sequences SET value = ? WHERE name = 'issue_number'")
            .bind(next)
            .execute(&mut *tx)
            .await
            .map_err(|e| crate::Error::Storage(format!("Database error: {}", e)))?;
        
        tx.commit().await
            .map_err(|e| crate::Error::Storage(format!("Transaction error: {}", e)))?;
        
        Ok(next as u64)
    }
}