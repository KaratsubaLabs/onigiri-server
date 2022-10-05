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
        pub devices: Vec<Device>,
    }
}

pub mod auth {

    use serde::{Deserialize, Serialize};

    /*
    #[derive(Serialize, Deserialize)]
    pub struct RegisterBody<'r> {
        username: &'r str,
        password: &'r str,
    }
    */
}
