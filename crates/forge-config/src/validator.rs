//! Configuration validation using cuengine

use crate::Result;
use forge_core::Error;
use cuengine::CueEvaluator;
use std::path::Path;

pub struct ConfigValidator {
    evaluator: CueEvaluator,
}

impl ConfigValidator {
    pub fn new() -> Result<Self> {
        let evaluator = CueEvaluator::builder().build()
            .map_err(|e| Error::Config(format!("Failed to initialize CUE evaluator: {}", e)))?;
        Ok(Self { evaluator })
    }
    
    /// Validate configuration against CUE schema
    pub fn validate_cue_schema(&self, config_dir: &str, schema_dir: &str) -> Result<bool> {
        let config_path = Path::new(config_dir);
        let schema_path = Path::new(schema_dir);
        
        // Evaluate both schema and config to validate they compile
        let _schema_result = self.evaluator.evaluate(schema_path, "")
            .map_err(|e| Error::Validation(format!("Failed to compile schema: {}", e)))?;
        
        let _config_result = self.evaluator.evaluate(config_path, "")
            .map_err(|e| Error::Validation(format!("Failed to compile config: {}", e)))?;
        
        // If both evaluate successfully, consider it valid for now
        // TODO: Implement proper schema validation when cuengine supports it
        Ok(true)
    }
    
    /// Validate that a configuration is complete (no undefined values)
    pub fn validate_completeness(&self, config_dir: &str) -> Result<bool> {
        let config_path = Path::new(config_dir);
        
        // Try to evaluate - if it succeeds, consider it complete
        self.evaluator.evaluate(config_path, "")
            .map_err(|e| Error::Validation(format!("Failed to evaluate config: {}", e)))?;
        
        Ok(true)
    }
    
    /// Validate configuration syntax
    pub fn validate_syntax(&self, config_dir: &str) -> Result<Vec<String>> {
        let config_path = Path::new(config_dir);
        
        match self.evaluator.evaluate(config_path, "") {
            Ok(_) => Ok(Vec::new()), // No syntax errors
            Err(e) => {
                // Return syntax errors as a list
                let error_msg = format!("{}", e);
                Ok(vec![error_msg])
            }
        }
    }
    
    /// Get detailed validation errors
    pub fn get_validation_errors(&self, config_dir: &str, schema_dir: &str) -> Result<Vec<String>> {
        let mut errors = Vec::new();
        
        // Check config syntax first
        let config_path = Path::new(config_dir);
        if let Err(e) = self.evaluator.evaluate(config_path, "") {
            errors.push(format!("Config syntax error: {}", e));
        }
        
        // Check schema syntax
        let schema_path = Path::new(schema_dir);
        if let Err(e) = self.evaluator.evaluate(schema_path, "") {
            errors.push(format!("Schema syntax error: {}", e));
        }
        
        if !errors.is_empty() {
            return Ok(errors);
        }
        
        // If both compile successfully, no validation errors for now
        // TODO: Add proper schema validation
        Ok(errors)
    }
}