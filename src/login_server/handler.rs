use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use regex::Regex;

mod nt_crypto;

pub async fn handle_client(mut socket: TcpStream) {
    let mut buffer = vec![0; 1024];

    println!("Client connected: {:?}", socket.peer_addr());

    match socket.read(&mut buffer).await {
        Ok(0) => {
            println!("Connection closed by client: {:?}", socket.peer_addr());
            return;
        }
        Ok(n) => {
            let received = &buffer[..n];
            let decrypted_login_data = nt_crypto::NTCrypto::decode(received); // Decode received data
            println!("Received data: {}", decrypted_login_data);

            // Regex to match six non-whitespace sequences separated by whitespace
            let pattern = Regex::new(r"(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)").unwrap();

            if let Some(captures) = pattern.captures(&decrypted_login_data) {
                // Extract and print each part with consistent indexing
                for i in 1..=6 {
                    if let Some(part) = captures.get(i) {
                        println!("Part {}: {}", i, part.as_str());
                    }
                }

                // Extracting parts for further processing
                let username = captures.get(3).map_or("", |m| m.as_str());
                let password_hash = captures.get(4).map_or("", |m| m.as_str());
                println!("Debug: Raw password hash received: {}", password_hash);

                // Decrypting Part 4 (actual encrypted password hash)
                let decrypted_password = nt_crypto::NTCrypto::get_password(password_hash);
                println!("Decrypted password: {}", decrypted_password);

                // Validate username and password
                if username == "admin" && decrypted_password == "admin" {
                    let response = "NsTeST 37223 127.0.0.1:1337:0:1.1.Ancelloan:10000.10000.0\n";
                    if let Some(encrypted_response) = nt_crypto::NTCrypto::encrypt_login(response) {
                        println!("Sending response: {}", response.trim());
                        if let Err(e) = socket.write_all(&encrypted_response).await {
                            eprintln!("Failed to send response: {}", e);
                        }
                    } else {
                        eprintln!("Failed to encrypt response.");
                    }
                } else {
                    let response = "fail Your Login Data is wrong.\n";
                    if let Some(encrypted_failure_response) = nt_crypto::NTCrypto::encrypt_login(response) {
                        println!("Sending failure response: {}", response.trim());
                        if let Err(e) = socket.write_all(&encrypted_failure_response).await {
                            eprintln!("Failed to send failure response: {}", e);
                        }
                    } else {
                        eprintln!("Failed to encrypt failure response.");
                    }
                }
            } else {
                // Handle invalid format case
                let response = "fail Your Login Data is wrong.\n";
                if let Some(encrypted_failure_response) = nt_crypto::NTCrypto::encrypt_login(response) {
                    println!("Sending failure response: {}", response.trim());
                    if let Err(e) = socket.write_all(&encrypted_failure_response).await {
                        eprintln!("Failed to send failure response: {}", e);
                    }
                } else {
                    eprintln!("Failed to encrypt failure response.");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
        }
    }
}
