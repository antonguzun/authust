use actix_web::test;
use authust::handlers::api::roles::views::{
    RoleMemberBindingView, RolePermissionBindingView, RoleView,
};
use serde_json::json;

mod utils;
use utils::{
    init_test_service, test_delete, test_get, test_post, test_put, IntenalRoles::RoleAdmin,
};
mod constants;

#[actix_web::test]
async fn test_get_role() {
    let mut app = init_test_service().await;
    let req = test_get("/api/v1/roles/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let role: RoleView = test::read_body_json(resp).await;
    assert_eq!(role.role_name, "ROLE_AUTH_STAFF");
    assert_eq!(role.is_deleted, false);
}

#[actix_web::test]
async fn test_create_new_role() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "role_name": "ROLE_TEST",
    });
    let url = "/api/v1/roles";
    let req = test_post(url, RoleAdmin)
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 201);
    let role: RoleView = test::read_body_json(resp).await;
    assert_eq!(role.role_name, "ROLE_TEST");
    assert_eq!(role.is_deleted, false);
}

#[actix_web::test]
async fn test_delete_role() {
    let mut app = init_test_service().await;
    // delete row wich not existed
    let req = test_delete("/api/v1/roles/9999", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);

    // check initial row state in db
    let req = test_get("/api/v1/roles/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let role: RoleView = test::read_body_json(resp).await;
    assert_eq!(role.role_name, "ROLE_AUTH_STAFF");
    assert_eq!(role.is_deleted, false);
    assert_eq!(role.created_at, role.updated_at);

    // delete
    let req = test_delete("/api/v1/roles/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);
    // delete again
    let req = test_delete("/api/v1/roles/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);

    // check row updated in db
    let req = test_get("/api/v1/roles/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let role: RoleView = test::read_body_json(resp).await;
    assert_eq!(role.role_name, "ROLE_AUTH_STAFF");
    assert_eq!(role.is_deleted, true);
    assert_ne!(role.created_at, role.updated_at);
}

#[actix_web::test]
async fn test_bind_permission_with_role() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "role_id": 3,
        "permission_id": 1,
    });
    let url = "/api/v1/roles/bind_permisson";
    let req = test_put(url, RoleAdmin).set_json(request_body).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: RolePermissionBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.role_id, 3);
    assert_eq!(binding.permission_id, 1);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_bind_permission_with_role_but_binding_exists() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "role_id": 1,
        "permission_id": 1,
    });
    let url = "/api/v1/roles/bind_permisson";
    let req = test_put(url, RoleAdmin).set_json(request_body).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: RolePermissionBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.role_id, 1);
    assert_eq!(binding.permission_id, 1);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_bind_permission_with_role_implicitly_enable_without_creation() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "role_id": 2,
        "permission_id": 3,
    });
    let url = "/api/v1/roles/bind_permisson";
    let req = test_put(url, RoleAdmin).set_json(request_body).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: RolePermissionBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.role_id, 2);
    assert_eq!(binding.permission_id, 3);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_unbind_permission_with_role() {
    let mut app = init_test_service().await;
    let req = test_put("/api/v1/roles/1/unbind_permisson/1", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: RolePermissionBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.role_id, 1);
    assert_eq!(binding.permission_id, 1);
    assert_eq!(binding.is_deleted, true);
    assert_ne!(binding.created_at, binding.updated_at);
}

#[actix_web::test]
async fn test_bind_member_with_role() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "role_id": 3,
        "user_id": 1,
    });
    let url = "/api/v1/roles/bind_member";
    let req = test_put(url, RoleAdmin).set_json(request_body).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: RoleMemberBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.role_id, 3);
    assert_eq!(binding.user_id, 1);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_bind_member_with_role_but_binding_exists() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "role_id": 1,
        "user_id": 1,
    });
    let req = test_put("/api/v1/roles/bind_member", RoleAdmin)
        .set_json(request_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: RoleMemberBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.role_id, 1);
    assert_eq!(binding.user_id, 1);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_bind_member_with_role_implicitly_enable_without_creation() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "role_id": 3,
        "user_id": 2,
    });
    let url = "/api/v1/roles/bind_member";
    let req = test_put(url, RoleAdmin).set_json(request_body).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: RoleMemberBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.role_id, 3);
    assert_eq!(binding.user_id, 2);
    assert_eq!(binding.is_deleted, false);
}

#[actix_web::test]
async fn test_unbind_member_with_role() {
    let mut app = init_test_service().await;
    let req = test_put("/api/v1/roles/3/unbind_member/2", RoleAdmin).to_request();

    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let binding: RoleMemberBindingView = test::read_body_json(resp).await;
    assert_eq!(binding.role_id, 3);
    assert_eq!(binding.user_id, 2);
    assert_eq!(binding.is_deleted, true);
    assert_ne!(binding.created_at, binding.updated_at);
}
