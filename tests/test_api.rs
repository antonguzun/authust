use actix_http::Request;
use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, web, App};
use common::{Config, Resources};
use main::init_api_v1;


async fn clean_db(resources: &Resources) -> () {
    let client = resources.db_pool.get().await.unwrap();
    let stmt = client
            .prepare("TRUNCATE users")
            .await.unwrap();
    client.execute(&stmt, &[]).await.unwrap();
}

async fn init_test_service(
) -> impl Service<Request, Response = ServiceResponse<BoxBody>, Error = actix_web::Error> {
    let config = Config::create_config();
    let resources = Resources::create_resources(&config).await;
    clean_db(&resources).await.unwrap();
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
    let req = test::TestRequest::get().uri("/api/v1/user/1").to_request();
    let mut resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200)
}

#[actix_rt::test]
async fn test_get_entity_not_found() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/user/999991")
        .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_rt::test]
async fn test_get_entity_wrong_params() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/user/sadf")
        .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    // странно что web::Path приводит к 404 ошибке, а не к 400
    assert_eq!(resp.status(), 404);
}

#[actix_rt::test]
async fn test_delete_entity() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::delete()
        .uri("/api/v1/user/3")
        .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 204)
}

#[actix_rt::test]
async fn test_delete_entity_404() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::delete()
        .uri("/api/v1/user/999")
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
#[path = "../src/usecases.rs"]
mod usecases;
#[path = "../src/storage.rs"]
mod storage;
#[path = "../src/usecases/user.rs"]
mod user;