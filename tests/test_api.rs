use actix_web::{http::header, test};
use rust_crud::common::SecurityConfig;
use rust_crud::usecases::user::crypto::verificate_jwt;
use rust_crud::usecases::user::entities::{SingnedInfo, User};
use serde_json::json;

mod utils;
use utils::init_test_service;
mod constants;
use constants::TEST_BASIC_AUTH_HEADER;

#[actix_web::test]
async fn test_get_user() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get().uri("/api/v1/users/1").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200)
}

#[actix_web::test]
async fn test_get_user_not_found() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/users/999991")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_user_wrong_params() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/users/sadf")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_delete_user() {
    let mut app = init_test_service().await;

    let req = test::TestRequest::get().uri("/api/v1/users/3").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);

    let req = test::TestRequest::delete()
        .uri("/api/v1/users/3")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 204);

    let req = test::TestRequest::get().uri("/api/v1/users/3").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_delete_user_what_doesnt_exist() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::delete()
        .uri("/api/v1/users/999")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 204)
}

#[actix_web::test]
async fn test_create_new_user() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "username": "tester",
        "password": "test_pass",
    });
    let req = test::TestRequest::post()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/users")
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
    let wrong_headers = [
        ("Authorization", "dGVzdF91c2VyOmhlbGxvMQ=="), // wrong password
        ("Authorization", "dGVzdF91c2VyOg=="),         // no password
        ("Authorization", "dGVzdF91c2VyOmhlbGxv"),     // no divider
        ("Authorization", "OmhlbGxv"),                 // no username
        ("Authorization", "asd"),
        ("Authorization", ""),
    ];
    let mut app = init_test_service().await;
    for wrong_header in wrong_headers {
        let req = test::TestRequest::post()
            .insert_header(wrong_header)
            .uri("/api/v1/users/sign_in")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 403)
    }
}

#[actix_web::test]
async fn test_sign_in() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::post()
        .insert_header(TEST_BASIC_AUTH_HEADER)
        .uri("/api/v1/users/sign_in")
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
