//! wrapper around surrealdb http interface
use reqwest::blocking::*;

pub struct DB {
    pub database_url: String, 
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
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
}

#[cfg(test)]
mod tests {
    use super::DB;

    #[test]
    fn connection() {

        let db = DB {
            database_url: "http://localhost:8000".into(),
            namespace: "test".into(),
            database: "test".into(),
            username: "root".into(),
            password: "root".into(),
        };

        assert!(db.query("INFO FOR DB;").is_ok());
    }

}
