use serde::{Deserialize, Serialize};

// PacketHeader trait
pub trait PacketHeader {
    fn header() -> &'static str;
}

// Define the Nos0575Packet struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Nos0575Packet {
    pub number: i32,
    pub name: String,
    pub password: String,
    pub client_data: String,
}

impl PacketHeader for Nos0575Packet {
    fn header() -> &'static str {
        "NoS0575"
    }
}

