use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterBody<'r> {
    username: &'r str,
    password: &'r str,
}

#[post("/register", data = "<body>")]
pub fn register(body: Json<RegisterBody<'_>>) {}

#[post("/login")]
pub fn login() {}
