//! Authentication middleware

use crate::JwtManager;
use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(jwt_manager): State<JwtManager>,
    mut request: Request,
    next: Next,
) -> std::result::Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));
    
    if let Some(token) = auth_header {
        match jwt_manager.decode_token(token) {
            Ok(claims) => {
                request.extensions_mut().insert(claims);
                Ok(next.run(request).await)
            }
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}