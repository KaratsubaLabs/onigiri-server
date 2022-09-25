use std::path::PathBuf;
use rocket::serde::json::Json;
use serde::Deserialize;

use crate::db::db;

#[post("/device", data = "<body>")]
pub fn create(body: Json<()>) {
    unimplemented!()
}

/// Proxies post request to corresponding device
#[get("/device/<device_id>/<rest..>")]
pub fn control_get(device_id: PathBuf, rest: PathBuf) {

    // look up device ip
    let id = device_id.to_str().unwrap_or_default();
    let device = db().query_device_by_id(id);
    println!("{:?}", device);


}

#[cfg(test)]
mod tests {
    use crate::launch;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn control_get() {
        let client = Client::tracked(launch()).unwrap();
        let mut res = client.get("/v1beta/device/0/random").dispatch();
    }
}
