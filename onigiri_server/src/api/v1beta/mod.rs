use rocket::{http::Status, Route};

use crate::api::guards::ApiKeyGuard;

pub mod auth;
pub mod device;

#[get("/health")]
fn health() -> Status {
    Status::Ok
}

pub fn routes() -> Vec<Route> {
    routes![
        health,
        // auth::register,
        // auth::login,
        auth::create_apikey,
        device::event_test,
        device::event_listen,
        device::event_push,
        device::register,
        device::list,
        device::control_get,
        device::control_post,
        // device::client_event_listen,
        // device::client_event_push,
    ]
}
