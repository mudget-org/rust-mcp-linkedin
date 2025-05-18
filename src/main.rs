mod config;
mod linkedin;
mod mcp;
mod server;
mod utils;
use rmcp::ServiceExt;
use anyhow::Result;
use axum::response::IntoResponse;
use config::Config;
use tokio::io::{stdin, stdout};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    utils::logging::init_logging()?;

    // Load configuration
    let config = Config::new()?;
    info!("Configuration loaded successfully");

    // Create LinkedIn client
    let linkedin_client = linkedin::client::LinkedInClient::new(&config)?;
    info!("LinkedIn client initialized");

    // Create server handler
    let service = server::handler::LinkedInPostServer::new(linkedin_client);
    info!("LinkedIn MCP server handler created");

    // Create a closure that returns the linkedin_server
    // let service = move || linkedin_server.clone();

    let transport = (stdin(), stdout());

    // this call will finish the initialization process
    let server = service.serve(transport).await?;

    // let ct = SseServer::serve(config.server_address.parse()?)
    //     .await?
    //     .with_service(server_factory);

    tokio::signal::ctrl_c().await?;
    server.waiting().await?;
    // ct.cancel();
    Ok(())
}

// Health check endpoint
// async fn health_check() -> impl IntoResponse {
//     "LinkedIn MCP Server - Status: Running"
// }
