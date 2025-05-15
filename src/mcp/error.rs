use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum McpServerError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("LinkedIn API error: {0}")]
    LinkedInApiError(String),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Method not found: {0}")]
    MethodNotFound(String),
}

#[derive(Debug)]
pub struct LinkedInError(pub String);

impl fmt::Display for LinkedInError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LinkedIn API Error: {}", self.0)
    }
}
