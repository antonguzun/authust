use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use authust::apps::{init_api_v1, init_external_v1, init_internal_v1, init_system};
use authust::common::{Config, Resources};
use authust::middlewares::bearer_validator;

use log::debug;
extern crate env_logger;

pub fn run_server(resources: Resources, config: Config) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(bearer_validator);
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(resources.clone()))
            .data(resources.clone())
            .service(web::scope("api/v1").configure(init_api_v1).wrap(auth))
            .service(web::scope("srv/v1").configure(init_internal_v1))
            .service(web::scope("auth/v1").configure(init_external_v1))
            .service(web::scope("").configure(init_system))
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
    debug!(target: "init", "{:#?}", config);
    run_server(resources, config)?.await
}
