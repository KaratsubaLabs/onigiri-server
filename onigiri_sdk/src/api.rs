//! API calls

use onigiri_types::db::ApiType;
use reqwest::{Client, Response};

use crate::client::API_KEY_HEADER;

pub trait Device {
    const API_TYPE: ApiType;
    fn new(api_url: &str, api_key: &str, id: &str) -> anyhow::Result<Box<Self>>;
    fn get_api_url(&self) -> &str;
    fn get_api_key(&self) -> &str;
    fn get_id(&self) -> &str;
}

pub struct LCDDevice {
    api_url: String,
    api_key: String,
    id: String,
}

impl LCDDevice {
    pub async fn write_line(&self, line: u8, content: &str) -> reqwest::Result<Response> {
        Client::new()
            .post(format!(
                "{}/device/{}/lcd/write/{}",
                self.get_api_url(),
                self.get_id(),
                line
            ))
            .body(content.to_owned())
            .header(API_KEY_HEADER, self.api_key.clone())
            .send()
            .await
    }

    pub async fn clear(&self) -> reqwest::Result<Response> {
        Client::new()
            .post(format!(
                "{}/device/{}/lcd/clear",
                self.get_api_url(),
                self.get_id()
            ))
            .header(API_KEY_HEADER, self.api_key.clone())
            .send()
            .await
    }
}

impl Device for LCDDevice {
    const API_TYPE: ApiType = ApiType::LCD;

    fn new(api_url: &str, api_key: &str, id: &str) -> anyhow::Result<Box<Self>> {
        Ok(Box::new(LCDDevice {
            api_url: api_url.to_owned(),
            api_key: api_key.to_owned(),
            id: id.to_owned(),
        }))
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_api_url(&self) -> &str {
        &self.api_url
    }

    fn get_api_key(&self) -> &str {
        &self.api_key
    }
}

pub struct LEDDevice {}

impl LEDDevice {}
