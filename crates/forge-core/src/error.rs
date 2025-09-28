//! Error types for Forge

use thiserror::Error;

/// Main error type for Forge operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("VCS error: {0}")]
    Vcs(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Policy error: {0}")]
    Policy(String),
    
    #[error("Search error: {0}")]
    Search(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;