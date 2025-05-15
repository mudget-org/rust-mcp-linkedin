use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinkedInClientError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("LinkedIn API error: {0}")]
    ApiError(String),
    
    #[error("HTTP client error: {0}")]
    HttpError(#[from] reqwest::Error),
}
