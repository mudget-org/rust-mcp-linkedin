use crate::linkedin::client::LinkedInClient;
use crate::mcp::error::LinkedInError;
use crate::mcp::models::{LinkedInPostRequest, LinkedInPostResponse};
use rmcp::tool;
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Clone)]
pub struct LinkedInTools {
    client: Arc<LinkedInClient>,
}

impl LinkedInTools {
    pub fn new(client: LinkedInClient) -> Self {
        Self { 
            client: Arc::new(client) 
        }
    }
    
    #[tool(description = "Create a LinkedIn post. Optionally schedule it for a future time.")]
    pub async fn create_post(
        &self, 
        #[tool(aggr)] req: LinkedInPostRequest
    ) -> Result<LinkedInPostResponse, LinkedInError> {
        info!("MCP Tool 'create_post' called");
        
        // Parse schedule time if provided
        let schedule_time = match req.schedule_time {
            Some(time_str) => {
                match time_str.parse::<chrono::DateTime<chrono::Utc>>() {
                    Ok(time) => Some(time),
                    Err(e) => {
                        warn!("Invalid schedule time format: {}", e);
                        return Err(LinkedInError(
                            "Invalid schedule time format. Use ISO 8601 format.".to_string()
                        ));
                    }
                }
            }
            None => None,
        };
        
        // Create the LinkedIn post
        match self.client.create_post(req.content, schedule_time).await {
            Ok(post_id) => {
                info!("Successfully created LinkedIn post");
                Ok(LinkedInPostResponse {
                    success: true,
                    post_id: Some(post_id),
                    message: "Post created successfully".to_string(),
                })
            }
            Err(e) => {
                warn!("Failed to create LinkedIn post: {}", e);
                Err(LinkedInError(format!("Failed to create post: {}", e)))
            }
        }
    }
}
