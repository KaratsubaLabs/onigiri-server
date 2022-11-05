use anyhow::anyhow;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use onigiri_types::db::ApiKeyRole;

use crate::db::db;

/// Request guard that ensures that a valid user API key is included in the `X-API-KEY` header
#[derive(Default)]
pub struct UserApiKeyGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserApiKeyGuard {
    type Error = anyhow::Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        api_key_guard::<UserApiKeyGuard>(request, ApiKeyRole::User).await
    }
}

/// Request guard that ensures that a valid device API key is included in the `X-API-KEY` header
#[derive(Default)]
pub struct DeviceApiKeyGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DeviceApiKeyGuard {
    type Error = anyhow::Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        api_key_guard::<DeviceApiKeyGuard>(request, ApiKeyRole::Device).await
    }
}

// NOTE: stupid function to parameterize parsing with enum
async fn api_key_guard<T: Default>(request: &Request<'_>, api_role: ApiKeyRole) -> Outcome<T, anyhow::Error> {
    let api_key_value = if let Some(api_key_value) = request.headers().get_one("X-API-KEY") {
        api_key_value
    } else {
        return Outcome::Failure((Status::Unauthorized, anyhow!("no api key found")));
    };

    // use a hardcoded valid API key for testing purposes
    #[cfg(feature = "debug")]
    if api_key_value == "API_KEY" {
        return Outcome::Success(T::default());
    }

    let api_key = if let Ok(api_key) = db().query_apikey_by_id(api_key_value).await {
        api_key
    } else {
        return Outcome::Failure((Status::Unauthorized, anyhow!("invalid api key")));
    };

    // check apikey role
    if api_key.role != api_role {
        return Outcome::Failure((Status::Unauthorized, anyhow!("incorrect role")));
    }

    // TODO delete header after (not sure if theres a need)

    Outcome::Success(T::default())
}
