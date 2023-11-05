use std::{env, str::FromStr};

#[derive(Debug, Clone)]
pub struct Config {
    pub eservice_url: String,
    pub server_port: u16,
    pub db_host: String,
    pub db_port: u16,
    pub db_dbname: String,
    pub db_username: String,
    pub db_password: String,
    pub authgodtoken: String,
}

impl Config {
    pub fn build() -> Result<Config, <u16 as FromStr>::Err> {
        let eservice_url = env::var("ESERVICE_URL").unwrap_or("http://127.0.0.1".into());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or("8081".into())
            .parse::<u16>()?;
        let db_host = env::var("POSTGRES_HOST").unwrap_or("127.0.0.1".into());
        let db_port = env::var("POSTGRES_PORT")
            .unwrap_or("5432".into())
            .parse::<u16>()?;
        let db_dbname = env::var("POSTGRES_DB").unwrap_or("master".into());
        let db_username = env::var("POSTGRES_USER").unwrap_or("sa".into());
        let db_password = env::var("POSTGRES_PASSWORD").unwrap_or("password".into());
        let authgodtoken = env::var("AUTHGODTOKEN").unwrap_or("BYPASS".into());

        Ok(Config {
            eservice_url,
            server_port,
            db_host,
            db_port,
            db_dbname,
            db_username,
            db_password,
            authgodtoken,
        })
    }
}