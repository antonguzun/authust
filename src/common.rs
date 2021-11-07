use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub service_name: String,
}

impl Config {
    pub fn create_config() -> Config {
        Config {
            database_url: env::var("DATABASE_URL").unwrap(),
            service_name: env::var("SERVICE_NAME").unwrap(),
        }
    }
}