use anyhow::anyhow;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::db::db;

/// Request guard that ensures that a valid API key is included in the `X-API-KEY` header
pub struct ApiKeyGuard;

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for ApiKeyGuard {
    type Error = anyhow::Error;
    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let api_key = if let Some(api_key) = request.headers().get_one("X-API-KEY") {
            api_key
        } else {
            return Outcome::Failure((Status::Unauthorized, anyhow!("no api key found")));
        };

        if db().query_apikey_by_id(api_key).await.is_err() {
            return Outcome::Failure((Status::Unauthorized, anyhow!("invalid api key")));
        }

        Outcome::Success(ApiKeyGuard)
    }
}
