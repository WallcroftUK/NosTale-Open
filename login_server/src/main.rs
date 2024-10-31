use tokio;
use tokio::net::TcpListener;

mod handler; // This should be in the same directory

#[tokio::main]
async fn main() {
    if let Err(e) = run_login_server().await {
        eprintln!("Error running login server: {}", e);
    }
}

async fn run_login_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:4000").await?;
    println!("Login server running on 0.0.0.0:4000");
    
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handler::handle_client(socket)); // Handle each client in a new task
    }
}
