//! wrapper around surrealdb http interface
mod models;

use std::net::IpAddr;

use reqwest::blocking::*;

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
    fn query(&self, body: &str) -> reqwest::Result<Response> {
        Client::new()
            .post(format!("{}/sql", self.database_url))
            .header("Content-Type", "application/json")
            .header("NS", self.namespace.to_owned())
            .header("DB", self.database.to_owned())
            .basic_auth(self.username.to_owned(), Some(self.password.to_owned()))
            .body(body.to_owned())
            .send()
    }

    /// Initial database initalization step
    pub fn migrate(&self) -> reqwest::Result<Response> {
        self.query(&format!(
            r#"
            DEFINE TABLE users SCHEMAFULL;
            DEFINE FIELD username ON TABLE users TYPE string;
            DEFINE FIELD username ON TABLE password TYPE string;
        "#
        ))
    }

    // TODO query sanitization
    pub fn create_user(&self, username: &str, hased_password: &str) -> reqwest::Result<Response> {
        self.query(&format!(
            r#"CREATE users SET username="{0}", password="{1}""#,
            username, hased_password
        ))
    }

    pub fn create_device(&self, name: &str, ip_address: IpAddr) -> reqwest::Result<Response> {
        self.query(&format!(
            r#"CREATE devices SET name="{0}", ip_address="{1}""#,
            name,
            ip_address.to_string()
        ))
    }

    pub fn query_device_by_name(&self, name: &str) -> reqwest::Result<Response> {
        self.query(&format!(
            r#"SELECT * FROM devices WHERE (name == {0})"#,
            name
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::db;

    #[test]
    fn connection() {
        assert!(db().query("INFO FOR DB;").is_ok());
    }

    #[test]
    fn create_user() {
        assert!(db().create_user("bill", "abc123").is_ok());
    }
}
