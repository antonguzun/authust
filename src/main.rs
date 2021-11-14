use crate::common::{Config, Resources};
use crate::handlers::{create_entity, delete_entity, get_entities, get_entity};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use log::info;

extern crate env_logger;

pub fn init_api_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(create_entity)
        .service(get_entities)
        .service(get_entity)
        .service(delete_entity);
}

pub fn run_server(resources: Resources, config: Config) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .data(resources.clone())
            .service(web::scope("/api/v1").configure(init_api_v1))
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::create_config();
    let resources = Resources::create_resources(&config).await;
    env_logger::init();
    info!(target: "init", "{:#?}", config);
    run_server(resources, config)?.await
}

mod common;
mod handlers;
