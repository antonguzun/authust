use actix_web::test;
use authust::handlers::api::permissions::views::{PermissionListingView, PermissionView};
use serde_json::json;

mod utils;
use utils::{
    init_test_service, test_delete, test_get, test_post,
    IntenalRoles::{RoleAdmin, RoleStaff},
};
mod constants;

#[actix_web::test]
async fn test_get_permission() {
    let mut app = init_test_service().await;
    let req = test_get("/api/v1/permissions/2", RoleStaff).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let permission: PermissionView = test::read_body_json(resp).await;
    assert_eq!(permission.permission_name, "WRITE_PERMISSION");
    assert_eq!(permission.is_deleted, false);
}

#[actix_web::test]
async fn test_create_new_permission() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "permission_name": "test_permission",
    });
    let req = test_post("/api/v1/permissions", RoleAdmin)
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
async fn test_delete_permission() {
    let mut app = init_test_service().await;
    // delete row wich not existed
    let req = test_delete("/api/v1/permissions/9999", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);

    // check initial row state in db
    let req = test_get("/api/v1/permissions/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let permission: PermissionView = test::read_body_json(resp).await;
    assert_eq!(permission.permission_name, "READ_ROLE");
    assert_eq!(permission.is_deleted, false);
    assert_eq!(permission.created_at, permission.updated_at);

    // delete
    let req = test_delete("/api/v1/permissions/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);
    // delete again
    let req = test_delete("/api/v1/permissions/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);

    // check row updated in db
    let req = test_get("/api/v1/permissions/3", RoleAdmin).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let permission: PermissionView = test::read_body_json(resp).await;
    assert_eq!(permission.permission_name, "READ_ROLE");
    assert_eq!(permission.is_deleted, true);
    assert_ne!(permission.created_at, permission.updated_at);
}

struct FiltersTestCase<'a> {
    url: &'a str,
    quantity_of_permissions: usize,
    total: i64,
    offset: i64,
    limit: i64,
}

#[actix_web::test]
async fn test_get_permissions_listing() {
    let mut app = init_test_service().await;
    let test_cases = [
        FiltersTestCase {
            url: "/api/v1/permissions?role_id=1&is_deleted=false&limit=10&offset=0",
            quantity_of_permissions: 8,
            total: 8,
            offset: 0,
            limit: 10,
        },
        FiltersTestCase {
            url: "/api/v1/permissions",
            quantity_of_permissions: 11,
            total: 11,
            offset: 0,
            limit: 100,
        },
        FiltersTestCase {
            url: "/api/v1/permissions?offset=0&limit=100",
            quantity_of_permissions: 11,
            total: 11,
            offset: 0,
            limit: 100,
        },
        FiltersTestCase {
            url: "/api/v1/permissions?offset=0&limit=2",
            quantity_of_permissions: 2,
            total: 11,
            offset: 0,
            limit: 2,
        },
        FiltersTestCase {
            url: "/api/v1/permissions?offset=2",
            quantity_of_permissions: 9,
            total: 11,
            offset: 2,
            limit: 100,
        },
    ];
    for test_case in test_cases.into_iter() {
        let req = test_get(test_case.url, RoleAdmin).to_request();
        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        assert_eq!(status, 200);
        let resp_view: PermissionListingView = test::read_body_json(resp).await;
        assert_eq!(
            resp_view.permissions.len(),
            test_case.quantity_of_permissions
        );
        assert_eq!(resp_view.pagination.total, test_case.total);
        assert_eq!(resp_view.pagination.offset, test_case.offset);
        assert_eq!(resp_view.pagination.limit, test_case.limit);
    }
}

#[actix_web::test]
async fn test_get_permissions_listing_400() {
    let mut app = init_test_service().await;
    let test_cases = [
        "/api/v1/permissions?limit=10000",
        "/api/v1/permissions?offset=-1",
        "/api/v1/permissions?limit=0",
    ];
    for url in test_cases {
        let req = test_get(url, RoleAdmin).to_request();
        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        assert_eq!(status, 400);
    }
}
