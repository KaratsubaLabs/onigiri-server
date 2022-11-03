use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

use onigiri_types::db::ApiKeyRole;

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
#[post("/apikey/user")]
pub(crate) async fn create_user_apikey() -> Result<Status, Status> {
    db().create_apikey(ApiKeyRole::User)
        .await
        .map_err(|f| Status::InternalServerError)?;
    Ok(Status::Ok)
}

#[post("/apikey/device")]
pub(crate) async fn create_device_apikey() -> Result<Status, Status> {
    db().create_apikey(ApiKeyRole::Device)
        .await
        .map_err(|f| Status::InternalServerError)?;
    Ok(Status::Ok)
}

#[cfg(test)]
mod tests {
    use super::create_user_apikey;

    #[tokio::test]
    async fn create() {
        let res = create_user_apikey().await;
        assert!(res.is_ok());
    }
}
