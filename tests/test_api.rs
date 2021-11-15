use actix_http::Request;
use actix_web::body::AnyBody;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, web, App};
use common::{Config, Resources};
use main::init_api_v1;

async fn init_test_service(
) -> impl Service<Request, Response = ServiceResponse<AnyBody>, Error = actix_web::Error> {
    let config = Config::create_config();
    let resources = Resources::create_resources(&config).await;
    test::init_service(
        App::new()
            .app_data(config.clone())
            .data(resources.clone())
            .service(web::scope("/api/v1").configure(init_api_v1)),
    )
    .await
}

#[actix_rt::test]
async fn test_get_entity() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/entity/1")
        .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200)
}

#[actix_rt::test]
async fn test_get_entity_not_found() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/entity/999991")
        .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_rt::test]
async fn test_get_entity_wrong_params() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/entity/sadf")
        .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    // странно что web::Path приводит к 404 ошибке, а не к 400
    assert_eq!(resp.status(), 404);
}

#[actix_rt::test]
async fn test_delete_entity() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::delete()
        .uri("/api/v1/entity/3")
        .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 204)
}

#[actix_rt::test]
async fn test_delete_entity_404() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::delete()
        .uri("/api/v1/entity/999")
        .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 204)
}

#[path = "../src/common.rs"]
mod common;
#[path = "../src/handlers.rs"]
mod handlers;
#[path = "../src/main.rs"]
mod main;
#[path = "../src/services.rs"]
mod services;
