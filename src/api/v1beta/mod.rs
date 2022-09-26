use rocket::Route;

pub mod auth;
pub mod device;

pub fn routes() -> Vec<Route> {
    routes![
        auth::register,
        auth::login,
        device::register,
        device::list,
        device::control_get
    ]
}
