#![allow(unused)]
#![allow(dead_code)]

pub mod api;

#[macro_use]
extern crate rocket;

#[launch]
fn launch() -> _ {
    rocket::build().register("/", catchers![api::handlers::not_found])
}
