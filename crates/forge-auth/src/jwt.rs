//! JWT token handling

use crate::{Claims, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtManager {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }
    
    pub fn encode_claims(&self, claims: &Claims) -> Result<String> {
        encode(&Header::default(), claims, &self.encoding_key)
            .map_err(|e| crate::Error::Auth(format!("JWT encoding error: {}", e)))
    }
    
    pub fn decode_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| crate::Error::Auth(format!("JWT decoding error: {}", e)))?;
        
        Ok(token_data.claims)
    }
}