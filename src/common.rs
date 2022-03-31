use std::env;

use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres;

#[derive(Clone, Debug)]
pub struct DbConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
    pub pool_max_size: usize,
}

#[derive(Clone, Debug)]
pub struct SecurityConfig {
    pub secret_key: String,
    pub expired_jwt_days: u32,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub database_config: DbConfig,
    pub security_config: SecurityConfig,
    pub service_name: String,
}

impl Config {
    pub fn create_config() -> Config {
        Config {
            database_config: DbConfig {
                user: env::var("PG_USER").expect("Expected env param PG_USER"),
                password: env::var("PG_PASSWORD").expect("Expected env param PG_PASSWORD"),
                host: env::var("PG_HOST").expect("Expected env param PG_HOST"),
                port: env::var("PG_PORT")
                    .expect("Expected env param PG_PORT")
                    .parse()
                    .expect("Wrong env param PG_PORT"),
                dbname: env::var("PG_DBNAME").expect("Expected env param PG_DBNAME"),
                pool_max_size: env::var("PG_POOL_MAX_SIZE")
                    .expect("Expected env param PG_POOL_MAX_SIZE")
                    .parse()
                    .expect("Wrong env param PG_POOL_MAX_SIZE"),
            },
            security_config: SecurityConfig {
                secret_key: env::var("SECRET_KEY").expect("Expected env param SECRET_KEY"),
                expired_jwt_days: env::var("EXPIRED_JWT_DAYS")
                    .expect("Expected env param EXPIRED_JWT_DAYS")
                    .parse()
                    .expect("Wrong env param EXPIRED_JWT_DAYS"),
            },
            service_name: env::var("SERVICE_NAME").expect("Expected env param SERVICE_NAME"),
        }
    }
}

#[derive(Clone)]
pub struct Resources {
    pub db_pool: Pool,
}

impl Resources {
    pub async fn create_resources(config: &Config) -> Resources {
        let db_pool = create_pool(&config);
        Resources { db_pool }
    }
}

fn create_pool(config: &Config) -> Pool {
    let mut pg_config = tokio_postgres::Config::new();
    pg_config.user(&config.database_config.user);
    pg_config.dbname(&config.database_config.dbname);
    pg_config.password(&config.database_config.password);
    pg_config.host(&config.database_config.host);
    pg_config.port(config.database_config.port);
    pg_config.application_name(&config.service_name);
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr)
        .max_size(config.database_config.pool_max_size)
        .build()
        .expect("Building postgres pool failured");
    pool
}
