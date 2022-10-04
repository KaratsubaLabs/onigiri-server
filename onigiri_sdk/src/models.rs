//! API connection object

use reqwest::Response;
use thiserror::Error;

use crate::api::Device;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not connect to server")]
    ConnectionRefused,
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
    // pub async fn get_devices(&self) -> reqwest::Result<Response> {
    //     let res = reqwest::Client::new()
    //         .get(format!("{}/device", self.api_url))
    //         .send()
    //         .await?;

    // }

    /// Claim a device to use
    pub async fn device<D: Device>(&self, device_id: &str) -> Result<(), Error> {
        // TODO check the type of the device
        D::API_TYPE;

        // check health of device
        let res = reqwest::Client::new()
            .get(format!("{}/device/{}/health", self.api_url, device_id))
            .send()
            .await;

        Ok(())
    }
}
