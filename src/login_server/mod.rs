pub mod handler;

use tokio::net::TcpListener;
use crate::config::load_config; 

pub async fn run_login_server() -> Result<(), Box<dyn std::error::Error>> {
    // let config = load_config()?; // Load the main configuration
    // let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;
    
    // println!("Login server running on {}:{}", config.host, config.port);
    let listener = TcpListener::bind("0.0.0.0:4000").await?;
    println!("Login server running on 0.0.0.0:4000");
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handler::handle_client(socket)); // Handle each client in a new task
    }
}
