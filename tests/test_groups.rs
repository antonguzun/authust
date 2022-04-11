use actix_web::{http::header, test};
use rust_crud::handlers::api::groups::views::{
    GroupView, GroupsMemberBindingView, GroupsPermissionBindingView,
};
use serde_json::json;

mod utils;
use utils::init_test_service;
mod constants;

#[actix_web::test]
async fn test_get_group() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/groups/1")
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
        .uri("/api/v1/groups")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 201);
    let group: GroupView = test::read_body_json(resp).await;
    assert_eq!(group.group_name, "test_group");
    assert_eq!(group.is_deleted, false);
}

#[actix_web::test]
async fn test_delete_group() {
    let mut app = init_test_service().await;
    // delete row wich not existed
    let req = test::TestRequest::delete()
        .uri("/api/v1/groups/9999")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);

    // check initial row state in db
    let req = test::TestRequest::get()
        .uri("/api/v1/groups/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let group: GroupView = test::read_body_json(resp).await;
    assert_eq!(group.group_name, "GROUP_1");
    assert_eq!(group.is_deleted, false);
    assert_eq!(group.created_at, group.updated_at);

    // delete
    let req = test::TestRequest::delete()
        .uri("/api/v1/groups/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);
    // delete again
    let req = test::TestRequest::delete()
        .uri("/api/v1/groups/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);

    // check row updated in db
    let req = test::TestRequest::get()
        .uri("/api/v1/groups/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let group: GroupView = test::read_body_json(resp).await;
    assert_eq!(group.group_name, "GROUP_1");
    assert_eq!(group.is_deleted, true);
    assert_ne!(group.created_at, group.updated_at);
}

#[actix_web::test]
async fn test_bind_permission_with_group() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "group_id": 3,
        "permission_id": 1,
    });
    let req = test::TestRequest::put()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/groups/bind_permisson")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: GroupsPermissionBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.group_id, 3);
    assert_eq!(binding.permission_id, 1);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_bind_permission_with_group_but_binding_exists() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "group_id": 1,
        "permission_id": 1,
    });
    let req = test::TestRequest::put()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/groups/bind_permisson")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: GroupsPermissionBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.group_id, 1);
    assert_eq!(binding.permission_id, 1);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_bind_permission_with_group_implicitly_enable_without_creation() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "group_id": 2,
        "permission_id": 3,
    });
    let req = test::TestRequest::put()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/groups/bind_permisson")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: GroupsPermissionBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.group_id, 2);
    assert_eq!(binding.permission_id, 3);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_unbind_permission_with_group() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::put()
        .uri("/api/v1/groups/1/unbind_permisson/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: GroupsPermissionBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.group_id, 1);
    assert_eq!(binding.permission_id, 1);
    assert_eq!(binding.is_deleted, true);
    assert_ne!(binding.created_at, binding.updated_at);
}

#[actix_web::test]
async fn test_bind_member_with_group() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "group_id": 3,
        "user_id": 1,
    });
    let req = test::TestRequest::put()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/groups/bind_member")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: GroupsMemberBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.group_id, 3);
    assert_eq!(binding.user_id, 1);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_bind_member_with_group_but_binding_exists() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "group_id": 1,
        "user_id": 1,
    });
    let req = test::TestRequest::put()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/groups/bind_member")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: GroupsMemberBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.group_id, 1);
    assert_eq!(binding.user_id, 1);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_bind_member_with_group_implicitly_enable_without_creation() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "group_id": 2,
        "user_id": 3,
    });
    let req = test::TestRequest::put()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/groups/bind_member")
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: GroupsMemberBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.group_id, 2);
    assert_eq!(binding.user_id, 3);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_unbind_member_with_group() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::put()
        .uri("/api/v1/groups/1/unbind_member/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: GroupsMemberBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.group_id, 1);
    assert_eq!(binding.user_id, 1);
    assert_eq!(binding.is_deleted, true);
    assert_ne!(binding.created_at, binding.updated_at);
}
