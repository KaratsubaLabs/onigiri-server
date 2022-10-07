//! API calls

use onigiri_types::db::ApiType;
use reqwest::{Client, Response};

use crate::client::API_KEY_HEADER;

pub trait Device {
    const API_TYPE: ApiType;
    fn new(api_url: &str, api_key: &str, id: &str) -> Box<Self>;
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

    fn new(api_url: &str, api_key: &str, id: &str) -> Box<Self> {
        Box::new(LCDDevice {
            api_url: api_url.to_owned(),
            api_key: api_key.to_owned(),
            id: id.to_owned(),
        })
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

pub struct LightDevice {
    api_url: String,
    api_key: String,
    id: String,
}

impl LightDevice {
    pub async fn light_on(&self) -> reqwest::Result<Response> {
        Client::new()
            .post(format!(
                "{}/device/{}/light/on",
                self.get_api_url(),
                self.get_id()
            ))
            .header(API_KEY_HEADER, self.api_key.clone())
            .send()
            .await
    }

    pub async fn light_off(&self) -> reqwest::Result<Response> {
        Client::new()
            .post(format!(
                "{}/device/{}/light/off",
                self.get_api_url(),
                self.get_id()
            ))
            .header(API_KEY_HEADER, self.api_key.clone())
            .send()
            .await
    }
}

// TODO this is all duplicated
impl Device for LightDevice {
    const API_TYPE: ApiType = ApiType::LIGHT;

    fn new(api_url: &str, api_key: &str, id: &str) -> Box<Self> {
        Box::new(LightDevice {
            api_url: api_url.to_owned(),
            api_key: api_key.to_owned(),
            id: id.to_owned(),
        })
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
