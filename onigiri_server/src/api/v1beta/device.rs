use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

use log::debug;
use onigiri_types::{
    api::v1beta::device::*,
    db::{ApiType, Device},
};
use reqwest::Client;
use rocket::{futures::TryFutureExt, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db::db;

/// [Device Facing] A device can ping this endpoint to register themselves
// TODO device facing endpoints maybe should be under a different path?
// TODO, should technically be "device/<device_id>"? only issue is that this allows the device
// choose their own ids, which is not the most ideal (you can also easily impersonate devices).
// maybe give each device their own token after registering (sorta like JWT)
#[post("/device", data = "<body>")]
pub(crate) async fn register(body: Json<RegisterBody<'_>>) -> Result<Status, Status> {
    // check device existence (TODO not the best to use only name rn)
    if let Ok(devices) = db().query_device_by_name(body.name).await {
        if devices.len() > 0 {
            return Err(Status::Conflict);
        }
    }

    db().create_device(body.name, body.ip_address, body.api_type)
        .await
        .map_err(|f| Status::InternalServerError)?;

    Ok(Status::Ok)
}

/// [Device Facing]
// NOTE not sure if this will ever be used
#[delete("/device/<device_id>")]
pub(crate) async fn unregister(device_id: PathBuf) {
    unimplemented!()
}

/// [User Facing] Get a list of all registered devices and some information about them
#[get("/device")]
pub(crate) async fn list() -> Result<Json<ListResponse>, Status> {
    let mut devices = db()
        .query_devices()
        .await
        .map_err(|f| Status::InternalServerError)?;

    // TODO right now Device id is being returned as `device:<id>`, should really only be `<id>`
    // fixing it here is just temporary, other routes might also return the device from querying
    // the db and need to remember to fix it
    for mut device in devices.iter_mut() {
        let mut id_pair = device.id.split(':');
        let _table = id_pair.next().unwrap();
        let id = id_pair.next().unwrap();
        device.id = id.to_owned();
    }

    Ok(Json(ListResponse { devices }))
}

/// [User Facing] Proxies get request to corresponding device
#[get("/device/<device_id>/<rest..>")]
pub(crate) async fn control_get(device_id: PathBuf, rest: PathBuf) -> Result<Status, Status> {
    // look up device ip
    let id = device_id.to_str().unwrap_or_default();
    // TODO not all errors are 404
    let device = db()
        .query_device_by_id(id)
        .await
        .map_err(|f| Status::NotFound)?;
    // make request to device
    let url = format!("http://{0}/{1}", device.ip_address, rest.to_str().unwrap());

    debug!("making request to {}", url);
    let device_res = Client::new().get(url).send().await.unwrap();

    println!("{:?}", device_res);
    Ok(Status::new(device_res.status().as_u16()))
}

/// [User Facing] Proxies post request to corresponding device
#[post("/device/<device_id>/<rest..>", data = "<body>")]
pub(crate) async fn control_post(
    device_id: PathBuf,
    rest: PathBuf,
    body: String,
) -> Result<Status, Status> {
    let id = device_id.to_str().unwrap_or_default();
    // TODO not all errors are 404
    let device = db()
        .query_device_by_id(id)
        .await
        .map_err(|f| Status::NotFound)?;
    let url = format!("http://{0}/{1}", device.ip_address, rest.to_str().unwrap());

    debug!("making request to {}", url);
    let device_res = Client::new().post(url).body(body).send().await.unwrap();

    println!("{:?}", device_res);
    Ok(Status::new(device_res.status().as_u16()))
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use onigiri_types::{api::v1beta::device::*, db::ApiType};
    use rocket::{http::Status, local::blocking::Client};

    use crate::app;

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
