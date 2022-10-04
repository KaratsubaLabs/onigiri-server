use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ApiType {
    LCD,
    LED,
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
