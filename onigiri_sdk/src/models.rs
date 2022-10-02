//! API connection object

use thiserror::Error;

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
}
