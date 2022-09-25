use rocket::Route;

pub mod auth;
pub mod device;

pub fn routes() -> Vec<Route> {
    routes![auth::register, auth::login, device::create, device::control_get]
}
