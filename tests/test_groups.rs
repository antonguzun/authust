use actix_web::{http::header, test};
use rust_crud::handlers::api::groups::views::GroupView;
use serde_json::json;

mod utils;
use utils::init_test_service;
mod constants;

#[actix_web::test]
async fn test_get_group() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/groups/1/")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let group: GroupView = test::read_body_json(resp).await;
    assert_eq!(group.group_name, "GROUP_1");
    assert_eq!(group.is_deleted, false);
}

#[actix_web::test]
async fn test_create_new_group() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "group_name": "test_group",
    });
    let req = test::TestRequest::post()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/groups/")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 201);
    let group: GroupView = test::read_body_json(resp).await;
    assert_eq!(group.group_name, "test_group");
    assert_eq!(group.is_deleted, false);
}
