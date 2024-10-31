use log::{info, error};

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Initialize the logger
    info!("Logging initialized.");
    Ok(())
}