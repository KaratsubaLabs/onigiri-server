use rocket::{http::Status, Route};

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
        auth::create_user_apikey,
        auth::create_device_apikey,
        device::register,
        device::list,
        device::control_get,
        device::control_post
    ]
}
