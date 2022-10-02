//! wrapper around surrealdb http interface
pub mod models;

use std::net::{IpAddr, Ipv4Addr};

use log::{debug, info};
use reqwest::Client;
use rocket::http::Status;
use serde::{de::DeserializeOwned, ser};
use serde_json::Value;
use thiserror::Error;

use self::models::Device;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Request to db errored {0:?}")]
    ReqwestError(reqwest::Error),
    #[error("Request to db failed with code {0:?}")]
    DBResponseNotOk(Status),
    #[error("Failed to parse db response body")]
    BodyParseFailed,
    #[error("No records were returned")]
    NoRecords,
}

pub type Result<T> = std::result::Result<T, DBError>;

pub struct DB {
    pub database_url: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

pub fn db() -> DB {
    DB {
        database_url: "http://localhost:8000".into(),
        namespace: "test".into(),
        database: "test".into(),
        username: "root".into(),
        password: "root".into(),
    }
}

impl DB {
    async fn query(&self, body: &str) -> Result<String> {
        debug!("query: {}", body);
        let res = Client::new()
            .post(format!("{}/sql", self.database_url))
            .header("Content-Type", "application/json")
            .header("NS", self.namespace.to_owned())
            .header("DB", self.database.to_owned())
            .basic_auth(self.username.to_owned(), Some(self.password.to_owned()))
            .body(body.to_owned())
            .send()
            .await
            .map_err(|e| DBError::ReqwestError(e))?;

        let status = Status::new(res.status().as_u16());
        if status != Status::Ok {
            return Err(DBError::DBResponseNotOk(status));
        }

        let body = res.text().await.map_err(|e| DBError::BodyParseFailed)?;
        let value: Value = serde_json::from_str(&body).map_err(|f| DBError::BodyParseFailed)?;

        if let Value::Array(arr) = value {
            let result = arr
                .get(0)
                .and_then(|r| r.get("result"))
                .ok_or(DBError::BodyParseFailed)?;
            Ok(result.to_string())
        } else {
            Err(DBError::BodyParseFailed)
        }
    }

    async fn query_typed<T: DeserializeOwned>(&self, body: &str) -> Result<T> {
        let res_str = self.query(body).await?;
        let parsed: T = serde_json::from_str(&res_str).map_err(|e| DBError::BodyParseFailed)?;
        Ok(parsed)
    }

    /// Initial database initalization step
    pub async fn migrate(&self) -> Result<()> {
        self.query(&format!(
            r#"
            DEFINE TABLE users SCHEMAFULL;
            DEFINE FIELD username ON TABLE users TYPE string;
            DEFINE FIELD username ON TABLE password TYPE string;
        "#
        ))
        .await?;
        Ok(())
    }

    // TODO query sanitization
    pub async fn create_user(&self, username: &str, hased_password: &str) -> Result<()> {
        self.query(&format!(
            r#"CREATE users SET username="{0}", password="{1}";"#,
            username, hased_password
        ))
        .await?;
        Ok(())
    }

    pub async fn create_device(&self, name: &str, ip_address: Ipv4Addr) -> Result<()> {
        self.query(&format!(
            r#"CREATE devices SET name="{0}", ip_address="{1}";"#,
            name,
            ip_address.to_string()
        ))
        .await?;
        Ok(())
    }

    pub async fn query_device_by_name(&self, name: &str) -> Result<Vec<Device>> {
        self.query_typed::<Vec<Device>>(&format!(
            r#"SELECT * FROM devices WHERE (name == "{0}");"#,
            name
        ))
        .await
    }

    /// Returns a single record
    pub async fn query_device_by_id(&self, id: &str) -> Result<Device> {
        let mut res = self
            .query_typed::<Vec<Device>>(&format!(r#"SELECT * FROM devices:{0}"#, id))
            .await?;

        if res.is_empty() {
            Err(DBError::NoRecords)
        } else {
            Ok(res.remove(0))
        }
    }

    pub async fn query_devices(&self) -> Result<Vec<Device>> {
        self.query_typed::<Vec<Device>>(&format!(r#"SELECT * FROM devices;"#,))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::db;

    // TODO async tests
    /*
    #[test]
    async fn connection() {
        assert!(db().query("INFO FOR DB;").await.is_ok());
    }

    #[test]
    async fn create_user() {
        assert!(db().create_user("bill", "abc123").await.is_ok());
    }
    */
}
