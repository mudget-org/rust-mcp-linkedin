use super::error::LinkedInClientError;
use super::models::*;
use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::{Client, header};
use std::time::Duration;
use tracing::{debug, error, info};

#[derive(Clone)]
pub struct LinkedInClient {
    http_client: Client,
    person_id: String,
    debug_mode: bool,
}

impl LinkedInClient {
    pub fn new(config: &Config) -> Result<Self, LinkedInClientError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", config.linkedin_access_token))
                .map_err(|_| {
                    LinkedInClientError::ConfigError("Failed to create auth header".to_string())
                })?,
        );
        
        // Add content-type header
        headers.insert(
            header::CONTENT_TYPE, 
            header::HeaderValue::from_static("application/json")
        );

        let http_client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| LinkedInClientError::HttpError(e))?;

        info!("LinkedIn client created for person ID: {}", config.linkedin_person_id);
        if config.debug_mode {
            info!("LinkedIn client running in DEBUG mode - posts will be simulated");
        }

        Ok(Self {
            http_client,
            person_id: config.linkedin_person_id.clone(),
            debug_mode: config.debug_mode,
        })
    }

    pub async fn create_post(
        &self,
        content: String,
        schedule_time: Option<DateTime<Utc>>,
    ) -> Result<String, LinkedInClientError> {
        let api_url = "https://api.linkedin.com/v2/ugcPosts";

        info!("Creating LinkedIn post");
        if let Some(time) = &schedule_time {
            info!("Post scheduled for: {}", time);
        }
        debug!("Post content: {}", content);

        // In debug mode, don't actually post to LinkedIn
        if self.debug_mode {
            info!("DEBUG MODE: Simulating successful LinkedIn post");
            tokio::time::sleep(Duration::from_millis(500)).await; // Simulate API call
            return Ok("debug-post-12345".to_string());
        }

        let payload = PostPayload {
            author: format!("urn:li:person:{}", self.person_id),
            lifecycleState: "PUBLISHED".to_string(),
            specificContent: SpecificContent {
                share_content: ShareContent {
                    shareCommentary: ShareCommentary { text: content },
                    shareMediaCategory: "NONE".to_string(),
                },
            },
            visibility: PostVisibility {
                visibility: "PUBLIC".to_string(),
            },
            scheduledAt: schedule_time.map(|t| t.timestamp_millis()),
        };

        debug!("Sending request to LinkedIn API: {:?}", payload);
        
        // Make the API request with detailed logging
        let response = match self.http_client.post(api_url).json(&payload).send().await {
            Ok(resp) => resp,
            Err(e) => {
                error!("HTTP error calling LinkedIn API: {}", e);
                return Err(LinkedInClientError::HttpError(e));
            }
        };

        let status = response.status();
        
        if !status.is_success() {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            error!("LinkedIn API error ({}): {}", status, error_body);
            return Err(LinkedInClientError::ApiError(format!("{}: {}", status, error_body)));
        }

        debug!("LinkedIn API returned successful response");
        
        // Parse the response
        let post_response = match response.json::<PostResponse>().await {
            Ok(resp) => resp,
            Err(e) => {
                error!("Error parsing LinkedIn API response: {}", e);
                return Err(LinkedInClientError::HttpError(e));
            }
        };

        info!("Successfully created LinkedIn post with ID: {}", post_response.id);
        Ok(post_response.id)
    }
}
