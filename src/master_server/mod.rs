// src/master_server/mod.rs
use tokio::sync::mpsc;
use std::io::{self, BufRead};
use crate::YourErrorType; // Import YourErrorType from the root

pub async fn run_master_console(command_sender: mpsc::Sender<String>) -> Result<(), YourErrorType> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    println!("Master console is running. Type commands:");

    for line in reader.lines() {
        let command = line.map_err(|_| YourErrorType)?; // Convert io error to your custom error type
        if command_sender.send(command).await.is_err() {
            println!("Failed to send command to servers.");
        }
    }

    Ok(())
}
