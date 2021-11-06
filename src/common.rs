use std::env;

#[derive(Clone)]
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
    pub fn display(&self) {
        println!("Config");
        println!("database_url={}", self.database_url);
        println!("service_name={}", self.service_name);
    }
}
