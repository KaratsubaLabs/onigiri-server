#![allow(unused)]
#![allow(dead_code)]

mod api;
mod db;
mod utils;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::builder().init();

    log::info!("Starting server...");

    app().launch().await?;

    Ok(())
}

fn app() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .register("/", api::handlers::catchers())
        .mount("/v1beta", api::v1beta::routes())
}
