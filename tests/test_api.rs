use actix_web::{body::Body, test, web, App};
use handlers::greetings;
use common::Config;

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
    assert_eq!(body, &Body::from("it is test_service"));
}

// #[actix_rt::test]
// async fn test_index_post() {
//     let mut app = test::init_service(App::new().route("/echo_event", web::get().to(index))).await;
//     let req = test::TestRequest::post().uri("/").to_request();
//     let resp = test::call_service(&mut app, req).await;
//     assert!(resp.status().is_client_error());
// }
#[path = "../src/common.rs"]
mod common;
#[path = "../src/handlers.rs"]
mod handlers;
