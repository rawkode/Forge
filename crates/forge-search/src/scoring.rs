//! Search result scoring

use crate::Result;

pub struct ScoreCalculator;

impl ScoreCalculator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn calculate_bm25_score(&self, _term_frequency: f32, _doc_frequency: f32) -> Result<f32> {
        todo!("Implement BM25 scoring")
    }
}