use crate::linkedin::client::LinkedInClient;
use crate::mcp::models::LinkedInPostRequest;
use async_trait::async_trait;
use rmcp::{ServerHandler, model::*, service::{Peer, RoleServer}, tool};
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Clone)]
pub struct LinkedInPostServer {
    client: Arc<LinkedInClient>,
    peer: Option<Peer<RoleServer>>,
}

impl LinkedInPostServer {
    pub fn new(client: LinkedInClient) -> Self {
        Self {
            client: Arc::new(client),
            peer: None,
        }
    }
}

#[tool(tool_box)]
impl LinkedInPostServer {
    #[tool(description = "Create a LinkedIn post. Optionally schedule it for a future time.")]
    async fn create_post(
        &self,
        #[tool(aggr)] req: LinkedInPostRequest,
    ) -> String {  // Simple string return
        info!("Tool 'create_post' called with content: {}", req.content);
        
        // Parse schedule time if provided
        let schedule_time = match req.schedule_time {
            Some(time_str) => match time_str.parse::<chrono::DateTime<chrono::Utc>>() {
                Ok(time) => {
                    info!("Post scheduled for: {}", time);
                    Some(time)
                },
                Err(e) => {
                    warn!("Invalid schedule time format: {}", e);
                    return serde_json::json!({
                        "success": false,
                        "post_id": null,
                        "message": "Invalid schedule time format. Use ISO 8601 format."
                    }).to_string();
                }
            },
            None => None,
        };
        
        // Create the LinkedIn post
        match self.client.create_post(req.content, schedule_time).await {
            Ok(post_id) => {
                info!("Successfully created LinkedIn post with ID: {}", post_id);
                serde_json::json!({
                    "success": true,
                    "post_id": post_id,
                    "message": "Post created successfully"
                }).to_string()
            }
            Err(e) => {
                warn!("Failed to create LinkedIn post: {}", e);
                serde_json::json!({
                    "success": false,
                    "post_id": null,
                    "message": format!("Failed to create post: {}", e)
                }).to_string()
            }
        }
    }
}

#[async_trait]
#[tool(tool_box)]
impl ServerHandler for LinkedInPostServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            // name: "LinkedIn Post Server".into(),
            // version: "1.0.0".into(),
            protocol_version: ProtocolVersion::V_2024_11_05,
            instructions: Some("A server for creating LinkedIn posts".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
    
    fn get_peer(&self) -> Option<Peer<RoleServer>> {
        self.peer.clone()
    }

    fn set_peer(&mut self, peer: Peer<RoleServer>) {
        self.peer = Some(peer);
    }
}
