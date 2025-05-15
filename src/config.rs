use anyhow::{Context, Result};
use dotenv::dotenv;
use std::env;

pub struct Config {
    pub linkedin_access_token: String,
    pub linkedin_person_id: String,
    pub server_address: String,
    pub debug_mode: bool,
}

impl Config {
    pub fn new() -> Result<Self> {
        // Load environment variables from .env file
        dotenv().ok();

        // Get required environment variables
        let linkedin_access_token = env::var("LINKEDIN_ACCESS_TOKEN")
            .context("LINKEDIN_ACCESS_TOKEN environment variable not set")?;

        let linkedin_person_id = env::var("LINKEDIN_PERSON_ID")
            .context("LINKEDIN_PERSON_ID environment variable not set")?;

        // Get server address with default
        let server_address =
            env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

        // Check if debug mode is enabled
        let debug_mode = env::var("DEBUG_MODE")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase()
            == "true";

        Ok(Self {
            linkedin_access_token,
            linkedin_person_id,
            server_address,
            debug_mode,
        })
    }
}
