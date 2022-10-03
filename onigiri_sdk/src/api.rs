//! API calls
use reqwest::{Client, Response};

// TODO move to API session struct or something
const API_URL: &'static str = "localhost:8080/v1beta";

trait Device {
    fn get_id(&self) -> &str;
}

pub struct LCDDevice {
    id: String,
}

impl LCDDevice {
    pub fn new(id: &str) -> Result<Self, anyhow::Error> {
        Ok(LCDDevice { id: id.to_owned() })
    }

    pub async fn write_line(&self, line: u8, content: String) -> reqwest::Result<Response> {
        Client::new()
            .post(format!(
                "{}/device/{}/lcd/write/{}",
                API_URL,
                self.get_id(),
                line
            ))
            .body(content)
            .send()
            .await
    }

    pub async fn clear(&self) -> reqwest::Result<Response> {
        Client::new()
            .post(format!("{}/device/{}/lcd/clear", API_URL, self.get_id()))
            .send()
            .await
    }
}

impl Device for LCDDevice {
    fn get_id(&self) -> &str {
        &self.id
    }
}

pub struct LEDDevice {}

impl LEDDevice {}

// general api calls
pub async fn get_devices() -> reqwest::Result<Response> {
    Client::new()
        .get(format!("{}/device", API_URL))
        .send()
        .await
}
