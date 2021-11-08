use std::env;

extern crate r2d2;
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

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

#[derive(Clone)]
pub struct Resources {
    pub db_pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl Resources {
    pub async fn create_resources(config: &Config) -> Resources {
        let manager = PostgresConnectionManager::new(config.database_url.parse().unwrap(), NoTls);
        let db_pool = r2d2::Pool::new(manager).unwrap();
        Resources { db_pool }
    }
}
