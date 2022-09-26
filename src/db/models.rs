use std::net::IpAddr;

pub struct User {
    pub username: String,
    pub hashed_password: String,
}

pub struct Device {
    pub name: String,
    pub ip_address: IpAddr
}
