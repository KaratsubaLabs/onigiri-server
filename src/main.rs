#![allow(unused)]
#![allow(dead_code)]

mod api;
mod utils;

#[macro_use]
extern crate rocket;

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/v1beta", api::v1beta::routes())
        .register("/", catchers![api::handlers::not_found])
}
