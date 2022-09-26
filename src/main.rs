#![allow(unused)]
#![allow(dead_code)]

mod api;
mod db;

#[macro_use]
extern crate rocket;

#[launch]
fn launch() -> _ {
    rocket::build().register("/", api::handlers::catchers())
        .mount("/v1beta", api::v1beta::routes())
}
