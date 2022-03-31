use actix_web::{http::header, test};
use rust_crud::common::SecurityConfig;
use rust_crud::usecases::user::crypto::verificate_jwt;
use rust_crud::usecases::user::entities::{SingnedInfo, User};
use serde_json::json;

mod utils;
use utils::init_test_service;
mod constants;
use constants::TEST_PASSWORD;

const WRONG_JWT_TOKEN: (&str, &str) = ("jwt-token", "i'm wrong");

#[actix_web::test]
async fn test_get_user() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get().uri("/api/v1/user/1").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200)
}

#[actix_web::test]
async fn test_get_user_not_found() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/user/999991")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_user_wrong_params() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/user/sadf")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_delete_user() {
    let mut app = init_test_service().await;

    let req = test::TestRequest::get().uri("/api/v1/user/3").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);

    let req = test::TestRequest::delete()
        .uri("/api/v1/user/3")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 204);

    let req = test::TestRequest::get().uri("/api/v1/user/3").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_delete_user_what_doesnt_exist() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::delete()
        .uri("/api/v1/user/999")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 204)
}

#[actix_web::test]
async fn test_create_new_user() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "username": "tester",
        "password": "test",
    });
    let req = test::TestRequest::post()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/user/")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    let user: User = test::read_body_json(resp).await;
    assert_eq!(user.username, "tester");
    assert_eq!(status, 201)
}

#[actix_web::test]
async fn test_sign_in_forbidden() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "username": "keker",
        "password": "wrong_passord",
    });
    let req = test::TestRequest::post()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/user/sign_in")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 403)
}

#[actix_web::test]
async fn test_sign_in() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "username": "Anton",
        "password": TEST_PASSWORD,
    });
    let req = test::TestRequest::post()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/user/sign_in")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let signed_info: SingnedInfo = test::read_body_json(resp).await;
    let conf = SecurityConfig {
        secret_key: String::from("some-secret"),
        expired_jwt_days: 14,
    };
    let jwt = verificate_jwt(&conf, &signed_info.jwt_token).unwrap();
    assert_eq!(jwt.user_id, 2);
}
