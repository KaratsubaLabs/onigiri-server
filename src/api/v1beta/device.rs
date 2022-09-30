use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

use log::debug;
use rocket::{futures::TryFutureExt, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::db::{db, models::ApiType};

#[derive(Serialize, Deserialize)]
pub struct RegisterBody<'r> {
    pub name: &'r str,
    pub ip_address: Ipv4Addr,
    pub api_type: ApiType,
}
/// [Device Facing] A device can ping this endpoint to register themselves
// TODO device facing endpoints maybe should be under a different path?
// TODO, should technically be "device/<device_id>"? only issue is that this allows the device
// choose their own ids, which is not the most ideal (you can also easily impersonate devices).
// maybe give each device their own token after registering (sorta like JWT)
#[post("/device", data = "<body>")]
pub async fn register(body: Json<RegisterBody<'_>>) -> Result<Status, Status> {
    let res = db()
        .create_device(body.name, body.ip_address)
        .await
        .map_err(|f| Status::InternalServerError)?;

    let status = Status::new(res.status().as_u16());
    if status != Status::Ok {
        // TODO handle this better
        return Err(Status::InternalServerError);
    }
    Ok(Status::Ok)
}

/// [Device Facing]
// NOTE not sure if this will ever be used
#[delete("/device/<device_id>")]
pub async fn unregister(device_id: PathBuf) {
    unimplemented!()
}

/// [User Facing] Get a list of all registered devices and some information about them
#[get("/device")]
pub async fn list() -> Result<Status, Status> {
    let res = db()
        .query_devices()
        .await
        .map_err(|f| Status::InternalServerError)?;

    let status = Status::new(res.status().as_u16());
    if status != Status::Ok {
        // TODO handle this better
        return Err(Status::InternalServerError);
    }
    let body = res.text().await.map_err(|f| Status::InternalServerError)?;
    debug!("{:?}", body);
    Ok(Status::Ok)
}

/// [User Facing] Proxies post request to corresponding device
#[get("/device/<device_id>/<rest..>")]
pub async fn control_get(device_id: PathBuf, rest: PathBuf) {
    // look up device ip
    let id = device_id.to_str().unwrap_or_default();
    let device = db().query_device_by_name(id).await;
    debug!("{:?}", device);
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use rocket::{http::Status, local::blocking::Client};

    use super::RegisterBody;
    use crate::{app, db::models::ApiType};

    #[test]
    fn control_get() {
        let client = Client::tracked(app()).unwrap();
        let mut res = client.get("/v1beta/device/0/random").dispatch();
    }

    #[test]
    fn register_device() {
        let client = Client::tracked(app()).unwrap();
        let mut res = client
            .post("/v1beta/device/")
            .json(&RegisterBody {
                name: "lcd",
                ip_address: Ipv4Addr::new(127, 0, 0, 1),
                api_type: ApiType::LCD,
            })
            .dispatch();
    }

    #[test]
    fn list_devices() {
        let client = Client::tracked(app()).unwrap();
        let mut res = client.get("/v1beta/device/").dispatch();
    }
}
