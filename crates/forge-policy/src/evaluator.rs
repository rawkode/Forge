//! Policy evaluation engine

use crate::{AuthorizationRequest, AuthorizationResult, Result};

pub struct PolicyEvaluator {
    engine: crate::cedar::CedarEngine,
}

impl PolicyEvaluator {
    pub fn new() -> Self {
        Self {
            engine: crate::cedar::CedarEngine::new(),
        }
    }
    
    pub async fn evaluate_request(&self, request: &AuthorizationRequest) -> Result<AuthorizationResult> {
        self.engine.evaluate(request)
    }
}