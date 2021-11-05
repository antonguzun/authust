use crate::common::{create_config, ConfigDisplay};
use crate::handlers::{
    create_entity, delete_entity, echo, echo_event, get_entities, get_entity, hello,
};
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = create_config();
    config.display();
    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .service(hello)
            .service(echo)
            .service(echo_event)
            .service(create_entity)
            .service(get_entities)
            .service(get_entity)
            .service(delete_entity)
            .route("/hey", web::get().to(handlers::greetings))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

mod common;
mod handlers;
