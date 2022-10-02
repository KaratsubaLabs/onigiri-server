use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};

// NOTE unused
pub struct User {
    pub username: String,
    pub hashed_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub ip_address: String,
    // pub api_type: String,
    // TODO deserialize from string to actual types
    // pub ip_address: Ipv4Addr,
    // pub api_type: ApiType,
}

#[derive(Serialize, Deserialize)]
pub enum ApiType {
    LCD,
    LED,
}
