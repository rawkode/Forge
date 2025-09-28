//! Static file serving

use axum::Router;
use tower_http::services::ServeDir;
use crate::server::AppState;

/// Create router for static files
pub fn create_static_router() -> Router<AppState> {
    // In production, static files should be served by a CDN or reverse proxy
    // This is just for development
    Router::new().nest_service("/", ServeDir::new("static"))
}