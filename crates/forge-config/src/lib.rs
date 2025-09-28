//! Configuration management for Forge
//!
//! This crate handles loading and unifying CUE configuration files using cuengine.
//! It provides the interface between CUE schemas and Rust types.

pub mod loader;
pub mod unifier;
pub mod validator;

pub use forge_core::{Error, Result};
pub use forge_core::config::*;
use cuengine::CueEvaluator;
use std::path::Path;

/// Configuration loader using cuengine
pub struct ConfigLoader {
    evaluator: CueEvaluator,
    config_root: String,
}

impl ConfigLoader {
    pub fn new(config_root: String) -> Result<Self> {
        let evaluator = CueEvaluator::builder().build()
            .map_err(|e| Error::Config(format!("Failed to initialize CUE evaluator: {}", e)))?;
        Ok(Self {
            evaluator,
            config_root,
        })
    }
    
    /// Load and validate Forge configuration from CUE files
    pub async fn load_forge_config(&self) -> Result<ForgeConfig> {
        let config_dir = Path::new(&self.config_root);
        self.load_and_parse_config(config_dir, "").await
    }
    
    /// Load repository configuration from forge.cue
    pub async fn load_repo_config(&self, repo_path: &str) -> Result<RepoConfig> {
        let config_dir = Path::new(repo_path);
        self.load_and_parse_config(config_dir, "").await
    }
    
    /// Load product configuration from CUE
    pub async fn load_product_config(&self, product_path: &str) -> Result<forge_core::types::ProductConfig> {
        let config_dir = Path::new(product_path);
        self.load_and_parse_config(config_dir, "").await
    }
    
    /// Generic configuration loading and parsing
    async fn load_and_parse_config<T>(&self, config_dir: &Path, package_name: &str) -> Result<T> 
    where
        T: serde::de::DeserializeOwned,
    {
        if !config_dir.exists() {
            return Err(Error::Config(format!("Configuration directory not found: {:?}", config_dir)));
        }
        
        // Evaluate CUE package using cuengine
        let json_str = self.evaluator.evaluate(config_dir, package_name)
            .map_err(|e| Error::Config(format!("Failed to evaluate CUE package: {}", e)))?;
        
        // Deserialize JSON to Rust type
        serde_json::from_str(&json_str)
            .map_err(|e| Error::Config(format!("Failed to deserialize config: {}", e)))
    }
    
    /// Validate CUE schema against given data
    pub fn validate_schema(&self, schema_dir: &str, data_dir: &str) -> Result<bool> {
        let schema_path = Path::new(schema_dir);
        let data_path = Path::new(data_dir);
        
        // Evaluate both schema and data
        let _schema_json = self.evaluator.evaluate(schema_path, "")
            .map_err(|e| Error::Validation(format!("Failed to compile schema: {}", e)))?;
        
        let _data_json = self.evaluator.evaluate(data_path, "")
            .map_err(|e| Error::Validation(format!("Failed to compile data: {}", e)))?;
        
        // For now, just check that both evaluate successfully
        // TODO: Implement proper schema validation logic
        Ok(true)
    }
}