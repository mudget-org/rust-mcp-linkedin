use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinkedInPostRequest {
    /// The text content for the LinkedIn post
    #[schemars(description = "The text content for the LinkedIn post")]
    pub content: String,
    
    /// Optional ISO 8601 timestamp for scheduling the post
    #[schemars(description = "Optional ISO 8601 timestamp for scheduling the post (ISO 8601 format)")]
    pub schedule_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinkedInPostResponse {
    /// Whether the post was successfully created
    pub success: bool,
    
    /// The ID of the created LinkedIn post if successful
    pub post_id: Option<String>,
    
    /// A message describing the result of the operation
    pub message: String,
}
