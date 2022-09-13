use rocket::Route;

pub mod auth;
pub mod device;

#[get("/health")]
fn health() -> String {
    format!("ok :)")
}

pub fn routes() -> Vec<Route> {
    routes![health]
}
