//! Forge Server - A modern code forge built with Rust and jj
//!
//! This is the main entry point for the Forge server application.

use anyhow::Result;
use clap::Parser;
use std::net::SocketAddr;
use tracing::{info, warn};

mod server;
mod config;
mod routes;
mod graphql;

use config::ServerConfig;

#[derive(Parser)]
#[command(name = "forge")]
#[command(about = "A modern code forge built with Rust and jj")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "forge.toml")]
    config: String,
    
    /// Server bind address
    #[arg(short, long, default_value = "0.0.0.0:3000")]
    bind: SocketAddr,
    
    /// Enable development mode
    #[arg(long)]
    dev: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    init_tracing()?;
    
    let cli = Cli::parse();
    
    info!("Starting Forge server");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    
    // Load configuration
    let config = ServerConfig::load(&cli.config).await?;
    
    if cli.dev {
        warn!("Running in development mode");
    }
    
    // Start the server
    server::start_server(config, cli.bind, cli.dev).await?;
    
    Ok(())
}

fn init_tracing() -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "forge_server=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();
    
    Ok(())
}