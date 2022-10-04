pub mod device {

    use std::net::{IpAddr, Ipv4Addr};

    use serde::{Deserialize, Serialize};

    use crate::db::{ApiType, Device};

    #[derive(Serialize, Deserialize)]
    pub struct RegisterBody<'r> {
        pub name: &'r str,
        pub ip_address: Ipv4Addr,
        pub api_type: ApiType,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ListResponse {
        devices: Vec<Device>,
    }
}
