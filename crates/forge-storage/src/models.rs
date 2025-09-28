//! Database models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database model for users
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<UserModel> for forge_core::types::User {
    fn from(model: UserModel) -> Self {
        Self {
            id: Uuid::parse_str(&model.id).unwrap(),
            email: model.email,
            name: model.name,
            avatar_url: model.avatar_url,
            created_at: DateTime::parse_from_rfc3339(&model.created_at).unwrap().with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&model.updated_at).unwrap().with_timezone(&Utc),
        }
    }
}

/// Database model for repositories
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RepositoryModel {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub default_branch: String,
    pub visibility: String,
    pub created_at: String,
    pub updated_at: String,
}

impl TryFrom<RepositoryModel> for forge_core::types::Repository {
    type Error = forge_core::Error;
    
    fn try_from(model: RepositoryModel) -> Result<Self, Self::Error> {
        let visibility = match model.visibility.as_str() {
            "private" => forge_core::types::Visibility::Private,
            "public" => forge_core::types::Visibility::Public,
            _ => return Err(forge_core::Error::Validation("Invalid visibility".to_string())),
        };
        
        Ok(Self {
            id: Uuid::parse_str(&model.id).map_err(|e| forge_core::Error::Validation(format!("Invalid UUID: {}", e)))?,
            slug: forge_core::types::Slug::new(&model.slug)?,
            name: model.name,
            description: model.description,
            default_branch: model.default_branch,
            visibility,
            created_at: DateTime::parse_from_rfc3339(&model.created_at)
                .map_err(|e| forge_core::Error::Validation(format!("Invalid timestamp: {}", e)))?
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&model.updated_at)
                .map_err(|e| forge_core::Error::Validation(format!("Invalid timestamp: {}", e)))?
                .with_timezone(&Utc),
        })
    }
}