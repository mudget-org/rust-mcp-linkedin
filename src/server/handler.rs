use crate::linkedin::client::LinkedInClient;
use crate::mcp::models::LinkedInPostResponse;
use crate::server::tools::LinkedInTools;
use async_trait::async_trait;
use rmcp::{
    ServerHandler,
    error::Error as McpError,
    model::*,
    service::{Peer, RequestContext, RoleServer},
    tool,
    tool_box
};
use std::sync::Arc;
use tracing::{debug, info};

#[derive(Clone)]
pub struct LinkedInPostServer {
    tools: Arc<LinkedInTools>,
    peer: Option<Peer<RoleServer>>,
}

impl LinkedInPostServer {
    pub fn new(client: LinkedInClient) -> Self {
        Self {
            tools: Arc::new(LinkedInTools::new(client)),
            peer: None,
        }
    }
}

#[async_trait]
#[tool(tool_box)]
impl ServerHandler for LinkedInPostServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            name: "LinkedInPostServer".into(),
            version: "1.0.0".into(),
            instructions: Some("A server for creating LinkedIn posts".into()),
            ..Default::default()
        }
    }

    async fn handle_request(
        &self,
        request: ClientRequest,
        context: RequestContext<RoleServer>,
    ) -> Result<ServerResult, McpError> {
        debug!("Received MCP request: {:?}", request);

        match request {
            ClientRequest::InitializeRequest(req) => {
                info!("Client connected: {:?}", req.params.client_info);

                // Return server info and capabilities
                Ok(ServerResult::InitializeResult(InitializeResult {
                    server_info: self.get_info(),
                    capabilities: ServerCapabilities::default(),
                }))
            }
            ClientRequest::ListToolsRequest(_) => {
                info!("Client requested tool list");

                // Use tool_box to handle tool listing
                tool_box!(@list_tools self)
            }
            ClientRequest::CallToolRequest(req) => {
                info!("Client wants to call tool: {}", req.params.name);

                match req.params.name.as_str() {
                    "create_post" => {
                        let result = self.tools.create_post(req.params.parameters).await;

                        match result {
                            Ok(response) => Ok(ServerResult::CallToolResult(CallToolResult {
                                contents: Contents::from_serializable(&response)?,
                            })),
                            Err(e) => Ok(ServerResult::CallToolResult(CallToolResult {
                                contents: Contents::Error(ErrorContents {
                                    code: "LINKEDIN_ERROR".to_string(),
                                    message: e.0,
                                    data: None,
                                }),
                            })),
                        }
                    }
                    _ => Err(McpError::method_not_found(&req.params.name)),
                }
            }
            _ => {
                // Handle other requests or return errors
                Err(McpError::method_not_found("Unknown request type"))
            }
        }
    }

    async fn handle_notification(&self, notification: ClientNotification) -> Result<(), McpError> {
        match notification {
            ClientNotification::CancelledNotification(note) => {
                info!(
                    "Client sent cancel for request ID: {:?}",
                    note.params.request_id
                );
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn get_peer(&self) -> Option<Peer<RoleServer>> {
        self.peer.clone()
    }

    fn set_peer(&mut self, peer: Peer<RoleServer>) {
        self.peer = Some(peer);
    }
}
