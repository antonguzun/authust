use actix_web::{body::Body, dev::Service, dev::ServiceResponse, test, web, App};
use common::{Config, Resources};
use main::init_api_v1;

// async fn init_service() -> ? {
//     let config = Config::create_config();
//     let resources = Resources::create_resources(&config).await;
//     let mut app = test::init_service(
//         App::new()
//             .app_data(config.clone())
//             .data(resources.clone())
//             .service(web::scope("/api/v1").configure(init_api_v1)),
//     ).await;
//     app
// }

#[actix_rt::test]
async fn test_get_entity() {
    let config = Config::create_config();
    let resources = Resources::create_resources(&config).await;
    let mut app = test::init_service(
        App::new()
            .app_data(config.clone())
            .data(resources.clone())
            .service(web::scope("/api/v1").configure(init_api_v1)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/entity/1")
        .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200)
}

#[actix_rt::test]
async fn test_get_entity_not_found() {
    let config = Config::create_config();
    let resources = Resources::create_resources(&config).await;
    let mut app = test::init_service(
        App::new()
            .app_data(config.clone())
            .data(resources.clone())
            .service(web::scope("/api/v1").configure(init_api_v1)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/entity/999991")
        .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[path = "../src/common.rs"]
mod common;
#[path = "../src/handlers.rs"]
mod handlers;
#[path = "../src/main.rs"]
mod main;
