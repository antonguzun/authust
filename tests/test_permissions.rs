use actix_web::{http::header, test};
use rust_crud::handlers::api::permissions::views::PermissionView;
use serde_json::json;

mod utils;
use utils::init_test_service;
mod constants;

#[actix_web::test]
async fn test_create_new_permission() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "permission_name": "test_permission",
    });
    let req = test::TestRequest::post()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/permissions/")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 201);
    let permission: PermissionView = test::read_body_json(resp).await;
    assert_eq!(permission.permission_name, "test_permission");
    assert_eq!(permission.is_deleted, false);
}

#[actix_web::test]
async fn test_delete_new_permission() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::delete()
        .uri("/api/v1/permissions/1/")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);
}
