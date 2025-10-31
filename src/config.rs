use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub mongodb_uri: String,
    pub database_name: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let mongodb_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017/".into());
        let database_name = env::var("DATABASE_NAME")
            .unwrap_or_else(|_| "RUSTY DB".into());
        
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("Port must be a number");

        Ok(Self {
            mongodb_uri,
            database_name,
            port,
        })
    }
}
