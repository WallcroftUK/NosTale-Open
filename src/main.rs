mod login_server;
mod master_server;
mod config;
mod logging;

use tokio::sync::mpsc;

#[derive(Debug)]
pub enum YourCommandType {
    Start,
    Stop,
}

#[derive(Debug)]
pub struct YourErrorType;

impl std::fmt::Display for YourErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "An error occurred")
    }
}

impl std::error::Error for YourErrorType {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging().expect("Failed to initialize logging");

    let (command_sender, _command_receiver) = mpsc::channel(32); // Create a command channel

    let login_server_future = login_server::run_login_server();
    let master_console_future = master_server::run_master_console(command_sender);

    let (login_result, master_console_result) = tokio::join!(login_server_future, master_console_future);

    // Handle login server result
    if let Err(e) = login_result {
        eprintln!("Error in login server: {:?}", e);
    }

    // Handle master console result
    if let Err(e) = master_console_result {
        eprintln!("Error in master console: {:?}", e);
    }

    Ok(())
}
