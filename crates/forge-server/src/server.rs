//! Main server implementation

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
};
use tracing::info;

use crate::config::ServerConfig;
use crate::routes;
use crate::graphql;

/// Start the Forge server
pub async fn start_server(
    config: ServerConfig,
    bind_addr: SocketAddr,
    dev_mode: bool,
) -> Result<()> {
    info!("Initializing server components");
    
    // Initialize database
    let db_pool = forge_storage::initialize_database(&config.storage).await
        .map_err(|e| anyhow::anyhow!("Failed to initialize database: {}", e))?;
    
    // Create GraphQL schema
    let schema = graphql::create_schema(db_pool.clone()).await?;
    
    // Build the application router
    let app = create_app_router(config.clone(), schema, dev_mode).await?;
    
    info!("Starting server on {}", bind_addr);
    
    // Start the server
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Create the main application router
async fn create_app_router(
    config: ServerConfig,
    schema: graphql::Schema,
    dev_mode: bool,
) -> Result<Router> {
    let cors = if dev_mode {
        CorsLayer::permissive()
    } else {
        CorsLayer::new()
            .allow_origin(config.server.cors_origins.iter().map(|s| s.parse().unwrap()).collect::<Vec<_>>())
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
            .allow_headers([axum::http::header::CONTENT_TYPE])
    };
    
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(config.server.request_timeout_seconds)))
        .layer(cors);
    
    let app = Router::new()
        // Health check endpoint
        .route("/health", get(routes::health::health_check))
        
        // GraphQL endpoint
        .route("/graphql", post(routes::graphql::graphql_handler))
        .route("/graphql", get(routes::graphql::graphql_playground))
        
        // API routes
        .route("/api/repositories", get(routes::repositories::list_repositories))
        
        // Static assets (in production, serve via CDN)
        .nest("/static", routes::static_files::create_static_router())
        
        // Apply middleware
        .layer(middleware)
        
        // Add application state
        .with_state(AppState {
            config,
            schema,
        });
    
    Ok(app)
}

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: ServerConfig,
    pub schema: graphql::Schema,
}