use anyhow::{Ok, Result};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init_logging() -> Result<()> {
    // Get log level from environment or use default
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    let level = match log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
