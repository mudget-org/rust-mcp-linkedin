mod config;
mod linkedin;
mod mcp;
mod server;
mod utils;

use anyhow::Result;
use axum::{Router, extract::State, response::Response, routing::get};
use config::Config;
use rmcp::service::server::serve_http_sse;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{Level, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    utils::logging::init_logging()?;

    // Load configuration
    let config = Config::new()?;

    // Create LinkedIn client
    let linkedin_client = linkedin::client::LinkedInClient::new(&config)?;

    // Create server handler
    let linkedin_server = Arc::new(server::handler::LinkedInPostServer::new(linkedin_client));

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "LinkedIn MCP Server" }))
        .route("/sse", get(handle_sse))
        .with_state(linkedin_server)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Run the server
    let addr: SocketAddr = config.server_address.parse()?;
    info!("Starting LinkedIn MCP server on {}", addr);

    // Use tokio's TcpListener for axum
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// SSE handler based on the example in axum_router.rs
async fn handle_sse(State(handler): State<Arc<server::handler::LinkedInPostServer>>) -> Response {
    serve_http_sse(handler)
}
