pub struct NTCrypto;

impl NTCrypto {
    // Encrypts the login packet
    pub fn encrypt_login(packet: &str) -> Option<Vec<u8>> {
        let mut bytes = Vec::from(packet.as_bytes()); // Convert string to bytes
        bytes.push(b' '); // Add a space at the end
    
        if bytes.is_empty() {
            return None; // Return None if the byte array is empty
        }
    
        for index in 0..bytes.len() - 1 {
            bytes[index] = (bytes[index] + 15) as u8; // Increment each byte by 15
        }
    
        // Instead of mutably borrowing, just modify the last index safely
        if let Some(last_byte) = bytes.last_mut() {
            *last_byte = 25; // Set the last byte to 25
        }
    
        Some(bytes) // Return the byte vector wrapped in Some
    }

// Decrypts the login packet
pub fn decode(bytes_buffer: &[u8]) -> String {
    let mut decrypted_packet = String::new();

    for &character in bytes_buffer {
        if character > 14 {
            // character > 14
            let transformed_char = (character - 0xF) ^ 0xC3;
            // Convert transformed_char to char using from_u32, which returns Option<char>
            if let Some(c) = std::char::from_u32(transformed_char.into()) {
                decrypted_packet.push(c); // Append the character
            } else {
                eprintln!("Transformed character value out of range: {}", transformed_char);
            }
        } else {
            // character <= 14
            let transformed_char = (256 - (0xF - character as u16)) ^ 0xC3; // Cast character to u16
            // Convert transformed_char to char using from_u32, which returns Option<char>
            if let Some(c) = std::char::from_u32(transformed_char.into()) {
                decrypted_packet.push(c); // Append the character
            } else {
                eprintln!("Transformed character value out of range: {}", transformed_char);
            }
        }
    }

    decrypted_packet
}
    // Extracts the password from the provided string
    pub fn get_password(password: &str) -> String {
        let str1 = if password.len() % 2 == 0 {
            &password[3..] // Remove the first 3 characters if even
        } else {
            &password[4..] // Remove the first 4 characters if odd
        };

        println!("Debug: str1: {}", str1);

        let mut empty = String::new();
        for index in (0..str1.len()).step_by(2) {
            empty.push(str1.chars().nth(index).unwrap());
        }

        println!("Debug: empty after first pass: {}", empty);

        if empty.len() % 2 != 0 {
            let str2 = &password[2..];
            empty.clear();

            for index in (0..str2.len()).step_by(2) {
                empty.push(str2.chars().nth(index).unwrap());
            }

            println!("Debug: empty after second pass: {}", empty);
        }

        let mut string_builder = String::new();
        for start_index in (0..empty.len()).step_by(2) {
            if start_index + 2 <= empty.len() {
                let hex_pair = &empty[start_index..start_index + 2];
                println!("Debug: Processing hex pair: {}", hex_pair);

                if let Ok(value) = u32::from_str_radix(hex_pair, 16) {
                    if let Some(c) = std::char::from_u32(value) {
                        string_builder.push(c);
                    } else {
                        println!("Debug: Conversion to char failed for value: {}", value);
                    }
                } else {
                    println!("Debug: Invalid hex value: {}", hex_pair);
                }
            }
        }

        println!("Debug: Final decrypted password: {}", string_builder);
        string_builder
    }

}
