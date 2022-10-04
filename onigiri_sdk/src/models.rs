//! API connection object

use anyhow::anyhow;
use onigiri_types::{api::v1beta::device::ListResponse, db};
use thiserror::Error;

use crate::api::Device;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not connect to server")]
    ConnectionRefused,
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
}

pub struct Client {
    pub api_url: String,
}

impl Client {
    /// Attempt to connect to the server
    pub fn connect(api_url: String) -> Result<Client, Error> {
        Ok(Client { api_url })
    }

    /// Get list of all devices that can be claimed
    pub async fn get_devices(&self) -> anyhow::Result<Vec<db::Device>> {
        let res = reqwest::Client::new()
            .get(format!("{}/device", self.api_url))
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
            .send()
            .await?;

        if !res.status().is_success() {
            // TODO error handle
        }

        let device = D::new(&self.api_url, device_id)?;
        Ok(*device)
    }
}
