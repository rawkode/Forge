//! CUE configuration unification using cuengine

use crate::Result;
use forge_core::Error;
use cuengine::CueEvaluator;
use std::path::Path;

pub struct ConfigUnifier {
    evaluator: CueEvaluator,
}

impl ConfigUnifier {
    pub fn new() -> Result<Self> {
        let evaluator = CueEvaluator::builder().build()
            .map_err(|e| Error::Config(format!("Failed to initialize CUE evaluator: {}", e)))?;
        Ok(Self { evaluator })
    }
    
    /// Unify multiple CUE configuration directories
    pub async fn unify_configs(&self, config_dirs: Vec<String>) -> Result<String> {
        if config_dirs.is_empty() {
            return Err(Error::Config("No configurations provided for unification".to_string()));
        }
        
        // For now, just evaluate the first directory
        // TODO: Implement proper unification when cuengine supports it
        let first_dir = Path::new(&config_dirs[0]);
        self.evaluator.evaluate(first_dir, "")
            .map_err(|e| Error::Config(format!("Failed to evaluate config: {}", e)))
    }
    
    /// Unify configurations with downward inheritance
    /// Parent configurations are overridden by child configurations
    pub async fn unify_with_inheritance(&self, _parent_config_dir: String, child_config_dir: String) -> Result<String> {
        // For now, just evaluate the child directory (it takes precedence)
        // TODO: Implement proper inheritance when cuengine supports unification
        let child_path = Path::new(&child_config_dir);
        self.evaluator.evaluate(child_path, "")
            .map_err(|e| Error::Config(format!("Failed to evaluate child config: {}", e)))
    }
}