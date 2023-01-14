#![allow(unused)]
#![allow(dead_code)]

use utils::state::{ClientPipe, DevPipe};

mod api;
mod db;
mod utils;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::builder().init();

    #[cfg(feature = "debug")]
    log::warn!("Debug mode is enabled, security is loosened.");

    log::info!("Starting server...");

    app().launch().await?;

    Ok(())
}

fn app() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(DevPipe::default())
        .manage(ClientPipe::default())
        .register("/", api::handlers::catchers())
        .mount("/v1beta", api::v1beta::routes())
}
