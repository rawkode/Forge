//! Configuration management for Forge
//!
//! This crate handles loading and unifying CUE configuration files.
//! It provides the interface between CUE schemas and Rust types.

pub mod loader;
pub mod unifier;
pub mod validator;

pub use forge_core::{Error, Result};
pub use forge_core::config::*;

/// Configuration loader
pub struct ConfigLoader {
    cue_binary_path: String,
    config_root: String,
}

impl ConfigLoader {
    pub fn new(cue_binary_path: String, config_root: String) -> Self {
        Self {
            cue_binary_path,
            config_root,
        }
    }
    
    /// Load and validate Forge configuration from CUE files
    pub async fn load_forge_config(&self) -> Result<ForgeConfig> {
        // Implementation will use CUE CLI to process configuration
        todo!("Implement CUE configuration loading")
    }
    
    /// Load repository configuration from forge.cue
    pub async fn load_repo_config(&self, _repo_path: &str) -> Result<RepoConfig> {
        // Implementation will use CUE CLI to process repository configuration
        todo!("Implement repository configuration loading")
    }
    
    /// Load product configuration from CUE
    pub async fn load_product_config(&self, _product_path: &str) -> Result<forge_core::types::ProductConfig> {
        // Implementation will use CUE CLI to process product configuration
        todo!("Implement product configuration loading")
    }
}