use std::path::PathBuf;

use rocket::serde::json::Json;
use serde::Deserialize;

use crate::db::db;

/// [Device Facing] A device can ping this endpoint to register themselves
// TODO device facing endpoints maybe should be under a different path?
#[post("/device/<device_id>", data = "<body>")]
pub fn register(device_id: PathBuf, body: Json<()>) {
    unimplemented!()
}

/// [Device Facing]
// NOTE not sure if this will ever be used
#[delete("/device/<device_id>")]
pub fn unregister(device_id: PathBuf) {
    unimplemented!()
}

/// [User Facing] Get a list of all registered devices and some information about them
#[get("/device")]
pub fn list() {
    unimplemented!()
}

/// [User Facing] Proxies post request to corresponding device
#[get("/device/<device_id>/<rest..>")]
pub fn control_get(device_id: PathBuf, rest: PathBuf) {
    // look up device ip
    let id = device_id.to_str().unwrap_or_default();
    let device = db().query_device_by_name(id);
    println!("{:?}", device);
}

#[cfg(test)]
mod tests {
    use rocket::{http::Status, local::blocking::Client};

    use crate::launch;

    #[test]
    fn control_get() {
        let client = Client::tracked(launch()).unwrap();
        let mut res = client.get("/v1beta/device/0/random").dispatch();
    }
}
