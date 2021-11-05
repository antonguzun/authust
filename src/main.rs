use actix_web::{
    delete, get, patch, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::env;

#[get("/")]
async fn hello(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[derive(Serialize, Deserialize)]
struct Event {
    id: i32,
    name: String,
}

#[post("/echo_event")]
async fn echo_event(event: web::Json<Event>) -> impl Responder {
    HttpResponse::Ok().json(Event {
        id: event.id,
        name: event.name.clone(),
    })
}

async fn greetings(req: HttpRequest) -> impl Responder {
    let c: &Config = req.app_data().unwrap();
    HttpResponse::Ok().body(format!("it is {}", c.service_name))
}

#[post("/entity")]
async fn create_entity(event: web::Json<Event>) -> impl Responder {
    HttpResponse::Ok().json(Event {
        id: event.id,
        name: event.name.clone(),
    })
}
#[delete("/entity/{id}")]
async fn delete_entity(id: web::Path<usize>) -> impl Responder {
    HttpResponse::Ok().body(format!("deleted {}", id))
}
#[get("/entity/{id}")]
async fn get_entity(id: web::Path<usize>) -> impl Responder {
    HttpResponse::Ok().body(format!("ok {}", id))
}
#[derive(Serialize, Deserialize)]
struct EntityQuery {
    text: String,
    offset: Option<usize>,
    limit: Option<usize>,
}
#[get("/entity")]
async fn get_entities(q: web::Query<EntityQuery>) -> impl Responder {
    HttpResponse::Ok().body(format!("search by text {}", q.text))
}

#[derive(Clone)]
struct Config {
    database_url: String,
    service_name: String,
}
pub trait ConfigDisplay {
    fn display(&self);
}
impl ConfigDisplay for Config {
    fn display(&self) {
        println!("Config");
        println!("database_url={}", self.database_url);
        println!("service_name={}", self.service_name);
    }
}
fn create_config() -> Config {
    Config {
        database_url: env::var("DATABASE_URL").unwrap(),
        service_name: env::var("SERVICE_NAME").unwrap(),
    }
}

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
            .route("/hey", web::get().to(greetings))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// #[cfg(test)]
// #[path = "./foo_test.rs"]
// mod api_test;

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{body::Body, test, web, App};

    #[actix_rt::test]
    async fn test_get_entity() {
        let data = Config {
            database_url: "432".to_string(),
            service_name: "test_service".to_string(),
        };

        let mut app = test::init_service(
            App::new()
                .app_data(data)
                .route("/hey", web::get().to(greetings)),
        )
        .await;
        let req = test::TestRequest::get().uri("/hey").to_request();
        let mut resp = test::call_service(&mut app, req).await;
        println!("{}", resp.status());
        assert!(resp.status().is_success());
        let body = resp.take_body();
        let body = body.as_ref().unwrap();
        assert_eq!(
            body,
            &Body::from("it is test_service")
        );
    }

    // #[actix_rt::test]
    // async fn test_index_post() {
    //     let mut app = test::init_service(App::new().route("/echo_event", web::get().to(index))).await;
    //     let req = test::TestRequest::post().uri("/").to_request();
    //     let resp = test::call_service(&mut app, req).await;
    //     assert!(resp.status().is_client_error());
    // }
}
