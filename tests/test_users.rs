use actix_web::test;

use authust::common::SecurityConfig;
use authust::usecases::users::crypto::decode_jwt;
use authust::usecases::users::entities::{SingnedInfo, User};

use serde_json::json;

mod utils;
use utils::{
    create_test_jwt, init_test_service, test_delete, test_get, test_post,
    IntenalRoles::{RoleAdmin, RoleStaff},
};
mod constants;
use constants::TEST_BASIC_AUTH_HEADER;

#[actix_web::test]
async fn test_get_user() {
    let mut app = init_test_service().await;
    let req = test_get("/api/v1/users/1", RoleStaff).to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200)
}

#[actix_web::test]
async fn test_get_user_not_found() {
    let mut app = init_test_service().await;
    let req = test_get("/api/v1/users/999991", RoleStaff).to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_user_wrong_params() {
    let mut app = init_test_service().await;
    let req = test_get("/api/v1/users/sadf", RoleStaff).to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_delete_user() {
    let mut app = init_test_service().await;

    let req = test_get("/api/v1/users/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);

    let req = test_delete("/api/v1/users/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 204);

    let req = test_get("/api/v1/users/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_delete_user_what_doesnt_exist() {
    let mut app = init_test_service().await;
    let req = test_delete("/api/v1/users/999", RoleAdmin).to_request();
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
    let req = test_post("/api/v1/users", RoleAdmin)
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 201);
    let user: User = test::read_body_json(resp).await;
    assert_eq!(user.username, "tester");
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
            .uri("/auth/v1/users/sign_in")
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
        .uri("/auth/v1/users/sign_in")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let signed_info: SingnedInfo = test::read_body_json(resp).await;
    let conf = SecurityConfig {
        secret_key: String::from("some-secret"),
        expired_jwt_days: 14,
    };
    let claims = decode_jwt(&conf, &signed_info.jwt_token).unwrap();
    assert_eq!(claims.user_id, 2);
    assert_eq!(
        claims.permissions,
        vec!["ROLE_AUTH_MANAGER", "ROLE_AUTH_STAFF", "ROLE_1", "ROLE_2"]
    );
}

#[actix_web::test]
async fn test_validate_jwt() {
    // create_test_jwt create fake perms in payload
    // test checks that /srv/v1/validate_jwt get perms from db by user_id
    let mut app = init_test_service().await;
    let jwt = create_test_jwt();
    let request_body = json!({
        "jwt_token": jwt,
    });
    let req = test::TestRequest::post()
        .uri("/srv/v1/validate_jwt")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let permissions: Vec<String> = test::read_body_json(resp).await;
    assert_eq!(
        permissions,
        vec![
            "ROLE_2",
            "ROLE_1",
            "READ_USER",
            "WRITE_PERMISSION",
            "ROLE_AUTH_ADMIN",
            "READ_PERMISSION",
            "PERM_1",
            "WRITE_USER",
            "PERM_2",
            "BIND_USER_WITH_ROLE",
            "WRITE_ROLE",
            "READ_ROLE",
            "BIND_ROLE_WITH_PERMISSION"
        ]
    );
}

#[actix_web::test]
async fn test_validate_jwt_wrong_token() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "jwt_token": "wrong_token",
    });
    let req = test::TestRequest::post()
        .uri("/srv/v1/validate_jwt")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 404);
}
