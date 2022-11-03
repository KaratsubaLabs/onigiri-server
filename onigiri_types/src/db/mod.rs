use pino_utils::enum_string;
use serde::{Deserialize, Serialize};

#[enum_string]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ApiType {
    LCD,
    LED,
    LIGHT,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKey {
    pub id: String,
    pub role: ApiKeyRole,
}

#[enum_string]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ApiKeyRole {
    User,
    Device
}

pub struct User {
    pub username: String,
    pub hashed_password: String,
}
