//! Configuration file loader with cuengine integration

use crate::Result;
use forge_core::Error;
use std::path::Path;
use tokio::fs;

pub struct FileLoader {
    root_path: String,
}

impl FileLoader {
    pub fn new(root_path: String) -> Self {
        Self { root_path }
    }
    
    /// Load CUE file content
    pub async fn load_cue_file(&self, path: &str) -> Result<String> {
        let full_path = Path::new(&self.root_path).join(path);
        
        if !full_path.exists() {
            return Err(Error::Config(
                format!("CUE file not found: {:?}", full_path)
            ));
        }
        
        fs::read_to_string(&full_path).await
            .map_err(|e| Error::Config(
                format!("Failed to read CUE file {:?}: {}", full_path, e)
            ))
    }
    
    /// Load multiple CUE files
    pub async fn load_cue_files(&self, paths: Vec<&str>) -> Result<Vec<String>> {
        let mut contents = Vec::new();
        
        for path in paths {
            let content = self.load_cue_file(path).await?;
            contents.push(content);
        }
        
        Ok(contents)
    }
    
    /// Check if CUE file exists
    pub fn cue_file_exists(&self, path: &str) -> bool {
        let full_path = Path::new(&self.root_path).join(path);
        full_path.exists()
    }
    
    /// Watch for configuration file changes
    pub async fn watch_for_changes(&self, _callback: Box<dyn Fn(&str) + Send + Sync>) -> Result<()> {
        // TODO: Implement file watching using tokio::fs::notify or similar
        // For now, return a placeholder implementation
        Err(Error::Config("File watching not yet implemented".to_string()))
    }
    
    /// List all CUE files in a directory
    pub async fn list_cue_files(&self, dir_path: &str) -> Result<Vec<String>> {
        let full_path = Path::new(&self.root_path).join(dir_path);
        
        if !full_path.is_dir() {
            return Err(Error::Config(
                format!("Directory not found: {:?}", full_path)
            ));
        }
        
        let mut cue_files = Vec::new();
        let mut entries = fs::read_dir(&full_path).await
            .map_err(|e| Error::Config(
                format!("Failed to read directory {:?}: {}", full_path, e)
            ))?;
        
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| Error::Config(
                format!("Failed to read directory entry: {}", e)
            ))? {
            
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "cue" {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_str) = file_name.to_str() {
                            cue_files.push(file_str.to_string());
                        }
                    }
                }
            }
        }
        
        Ok(cue_files)
    }
}