use anyhow::Result;
use my_logger::logger;
use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init_logger("logs", "demo.log", tracing::Level::DEBUG)?;

    info!("Logger initialized successfully");
    info!("This is an info message.");
    debug!("This is a debug message.");
    warn!("This is a warning message.");
    error!("This is an error message.");
    Ok(())
}
