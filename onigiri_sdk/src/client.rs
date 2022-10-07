//! API connection object

use anyhow::anyhow;
use onigiri_types::{api::v1beta::device::ListResponse, db};
use thiserror::Error;

use crate::api::Device;

pub const API_KEY_HEADER: &'static str = "X-API-KEY";

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not connect to server")]
    ConnectionRefused,
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
}

#[derive(Clone)]
pub struct ClientBuilder {
    api_url: String,
    api_key: String,
}

impl ClientBuilder {
    /// Attempt to connect to the server
    pub fn new(api_url: &str, api_key: &str) -> ClientBuilder {
        // TODO auth
        ClientBuilder {
            api_url: api_url.to_owned(),
            api_key: api_key.to_owned(),
        }
    }

    /// Attempt to connect to the server
    pub fn connect(&self) -> Result<Client, Error> {
        // TODO auth
        Ok(Client {
            api_url: self.api_url.to_owned(),
            api_key: self.api_key.to_owned(),
        })
    }
}

#[derive(Clone)]
pub struct Client {
    api_url: String,
    api_key: String,
}

impl Client {
    /// Get list of all devices that can be claimed
    pub async fn get_devices(&self) -> anyhow::Result<Vec<db::Device>> {
        let res = reqwest::Client::new()
            .get(format!("{}/device", self.api_url))
            .header(API_KEY_HEADER, self.api_key.clone())
            .send()
            .await?;

        let json: ListResponse = res.json().await?;

        Ok(json.devices)
    }

    /// Claim a device to use
    pub async fn device<D: Device>(&self, device_id: &str) -> anyhow::Result<D> {
        // check the type of the device
        // TODO not super efficient fetchign ALL the devices
        let devices = self.get_devices().await?;
        if devices
            .iter()
            .find(|d| d.id == device_id && d.api_type == D::API_TYPE)
            .is_none()
        {
            return Err(anyhow!(Error::DeviceNotFound(device_id.to_owned())));
        }

        // check health of device
        let res = reqwest::Client::new()
            .get(format!("{}/device/{}/health", self.api_url, device_id))
            .header(API_KEY_HEADER, self.api_key.clone())
            .send()
            .await?;

        if !res.status().is_success() {
            // TODO error handle
        }

        let device = D::new(&self.api_url, &self.api_key, device_id);
        Ok(*device)
    }

    /// Claim device to use without checking that the it is a valid device
    pub fn device_unchecked<D: Device>(&self, device_id: &str) -> D {
        let device = D::new(&self.api_url, &self.api_key, device_id);
        *device
    }
}

#[cfg(test)]
mod tests {

    use onigiri_types::db::ApiType;

    use super::{Client, ClientBuilder};
    use crate::api::LCDDevice;

    /// This test requires a running onigiri-server instance, with a registered lcd device (it's
    /// not the greatest test right now)
    #[tokio::test]
    async fn lcd_device() -> anyhow::Result<()> {
        let api_url = "http://127.0.0.1:8080/v1beta";
        let client = ClientBuilder::new(api_url, "API_KEY").connect()?;

        let devices = client.get_devices().await?;

        let lcd_device_id = devices
            .iter()
            .find(|d| d.api_type == ApiType::LCD)
            .expect("Could not find device with LCD api_type (this may not be this test's fault)");

        let lcd_device = client.device::<LCDDevice>(&lcd_device_id.id).await?;
        lcd_device.write_line(1, "hello world").await?;

        Ok(())
    }
}
