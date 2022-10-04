use pino_utils::enum_string;
use serde::{Deserialize, Serialize};

#[enum_string]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ApiType {
    LCD,
    LED,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub ip_address: String,
    pub api_type: ApiType,
    // TODO deserialize from string to actual types
    // pub ip_address: Ipv4Addr,
    // pub api_type: ApiType,
}

pub struct User {
    pub username: String,
    pub hashed_password: String,
}
