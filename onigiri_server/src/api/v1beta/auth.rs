use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::db::db;

/*
#[post("/register", data = "<body>")]
pub(crate) fn register(body: Json<RegisterBody<'_>>) {}

#[post("/login")]
pub(crate) fn login() {}
*/

/// Create a new API key
// TODO for now anyone can create API keys. in the future perhaps tie api keys to user accounts
// or have admin account approve of apikeys creation requests
#[post("/apikey")]
pub(crate) async fn create_apikey() -> Result<Status, Status> {
    db().create_apikey()
        .await
        .map_err(|f| Status::InternalServerError)?;
    Ok(Status::Ok)
}
