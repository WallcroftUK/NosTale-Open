pub struct NTCrypto;

impl NTCrypto {
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

fn main() {
    // Simulate test inputs for password decryption
    let test_passwords = [
        "4216D126F4568D26B9362E", // Example: Known passwords to test against
    ];
    
    for test_password in &test_passwords {
        println!("\nTesting password decryption for: {}", test_password);
        let decrypted_password = NTCrypto::get_password(test_password);
        println!("Decrypted password: {}", decrypted_password);
    }
}
