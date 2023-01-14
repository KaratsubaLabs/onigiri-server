#![allow(unused)]
#![allow(dead_code)]

use settings::SETTINGS_NO_API_KEY;
use utils::state::{ClientPipe, DevPipe};

mod api;
mod db;
mod settings;
mod utils;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::builder().init();

    if *SETTINGS_NO_API_KEY {
        log::warn!("Debug mode is enabled, security is loosened.");
    }

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
