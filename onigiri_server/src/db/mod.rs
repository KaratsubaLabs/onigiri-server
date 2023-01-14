//! wrapper around surrealdb http interface
use std::{
    env,
    net::{IpAddr, Ipv4Addr},
};

use lazy_static::lazy_static;
use log::{debug, info};
use onigiri_types::db::{ApiKey, ApiType, Device};
use reqwest::Client;
use rocket::http::Status;
use serde::{de::DeserializeOwned, ser};
use serde_json::Value;
use thiserror::Error;

use crate::utils::apikey::generate_apikey;

lazy_static! {
    static ref DB_URL: String =
        env::var("ONIGIRI_DB_URL").unwrap_or("http://127.0.0.1:8000".into());
    static ref DB_NAMESPACE: String = env::var("ONIGIRI_DB_NAMESPACE").unwrap_or("onigiri".into());
    static ref DB_NAME: String = env::var("ONIGIRI_DB_NAME").unwrap_or("onigiri".into());
    static ref DB_USERNAME: String = env::var("ONIGIRI_DB_USERNAME").unwrap_or("admin".into());
    static ref DB_PASSWORD: String = env::var("ONIGIRI_DB_PASSWORD").unwrap_or("password".into());
}

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
        database_url: DB_URL.to_string(),
        namespace: DB_NAMESPACE.to_string(),
        database: DB_NAME.to_string(),
        username: DB_USERNAME.to_string(),
        password: DB_PASSWORD.to_string(),
    }
}

impl DB {
    async fn query(&self, body: &str) -> Result<String> {
        debug!("query: {}", body);
        let res = Client::new()
            .post(format!("{}/sql", self.database_url))
            .header("Accept", "application/json")
            .header("NS", self.namespace.to_owned())
            .header("DB", self.database.to_owned())
            .basic_auth(self.username.to_owned(), Some(self.password.to_owned()))
            .body(body.to_owned())
            .send()
            .await
            .map_err(|e| DBError::ReqwestError(e))?;

        let status = Status::new(res.status().as_u16());
        if status != Status::Ok {
            log::error!("db error {:?}", res.text().await);
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

    // TODO decide difference between name and id
    pub async fn create_device(
        &self,
        name: &str,
        ip_address: Ipv4Addr,
        api_type: ApiType,
    ) -> Result<()> {
        self.query(&format!(
            r#"CREATE devices:{0} SET name="{0}", ip_address="{1}", api_type="{2}";"#,
            name,
            ip_address.to_string(),
            api_type.to_string(),
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

    pub async fn create_apikey(&self) -> Result<()> {
        // generate an apikey string
        let apikey = generate_apikey();

        // TODO maybe hash the apikey when storing in DB
        self.query(&format!(r#"CREATE apikeys:{0}"#, apikey))
            .await?;
        Ok(())
    }

    /// The api key is the id
    pub async fn query_apikey_by_id(&self, id: &str) -> Result<ApiKey> {
        let mut res = self
            .query_typed::<Vec<ApiKey>>(&format!(r#"SELECT * FROM apikeys:{0}"#, id))
            .await?;

        if res.is_empty() {
            Err(DBError::NoRecords)
        } else {
            Ok(res.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::db;
    use crate::db::DB_URL;

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

    #[test]
    fn read_db_conf() {
        println!("{}", DB_URL.as_str());
    }
}
