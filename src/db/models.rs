use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};

// NOTE unused
pub struct User {
    pub username: String,
    pub hashed_password: String,
}

// NOTE unused
pub struct Device {
    pub name: String,
    pub ip_address: Ipv4Addr,
}

#[derive(Serialize, Deserialize)]
pub enum ApiType {
    LCD,
    LED,
}
