use rocket::{Route, http::Status};

pub mod auth;
pub mod device;

#[get("/health")]
fn health() -> Status {
    Status::Ok
}

pub fn routes() -> Vec<Route> {
    routes![
        health,
        auth::register,
        auth::login,
        device::register,
        device::list,
        device::control_get
    ]
}
