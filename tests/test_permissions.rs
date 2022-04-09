use actix_web::{http::header, test};
use rust_crud::handlers::api::permissions::views::{PermissionListingView, PermissionView};
use serde_json::json;

mod utils;
use utils::init_test_service;
mod constants;

#[actix_web::test]
async fn test_get_permission() {
    let mut app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/api/v1/permissions/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let permission: PermissionView = test::read_body_json(resp).await;
    assert_eq!(permission.permission_name, "PERM_1");
    assert_eq!(permission.is_deleted, false);
}

#[actix_web::test]
async fn test_create_new_permission() {
    let mut app = init_test_service().await;
    let request_body = json!({
        "permission_name": "test_permission",
    });
    let req = test::TestRequest::post()
        .insert_header(header::ContentType::json())
        .uri("/api/v1/permissions")
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
    let req = test::TestRequest::delete()
        .uri("/api/v1/permissions/9999")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);

    // check initial row state in db
    let req = test::TestRequest::get()
        .uri("/api/v1/permissions/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let permission: PermissionView = test::read_body_json(resp).await;
    assert_eq!(permission.permission_name, "PERM_1");
    assert_eq!(permission.is_deleted, false);
    assert_eq!(permission.created_at, permission.updated_at);

    // delete
    let req = test::TestRequest::delete()
        .uri("/api/v1/permissions/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);
    // delete again
    let req = test::TestRequest::delete()
        .uri("/api/v1/permissions/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 204);

    // check row updated in db
    let req = test::TestRequest::get()
        .uri("/api/v1/permissions/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, 200);
    let permission: PermissionView = test::read_body_json(resp).await;
    assert_eq!(permission.permission_name, "PERM_1");
    assert_eq!(permission.is_deleted, true);
    assert_ne!(permission.created_at, permission.updated_at);
}

struct FiltersTestCase<'a> {
    url: &'a str,
    quantity_of_permissions: usize,
}

#[actix_web::test]
async fn test_get_permissions_listing() {
    let mut app = init_test_service().await;
    let test_cases = [
        FiltersTestCase {
            url: "/api/v1/permissions?group_id=1&is_deleted=false&limit=10&offset=0",
            quantity_of_permissions: 0,
        },
        FiltersTestCase {
            url: "/api/v1/permissions",
            quantity_of_permissions: 3,
        },
        FiltersTestCase {
            url: "/api/v1/permissions?offset=0&limit=100",
            quantity_of_permissions: 3,
        },
        FiltersTestCase {
            url: "/api/v1/permissions?offset=0&limit=2",
            quantity_of_permissions: 2,
        },
        FiltersTestCase {
            url: "/api/v1/permissions?offset=2",
            quantity_of_permissions: 1,
        },
    ];
    for test_case in test_cases.into_iter() {
        let req = test::TestRequest::get().uri(test_case.url).to_request();
        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        assert_eq!(status, 200);
        let resp_view: PermissionListingView = test::read_body_json(resp).await;
        assert_eq!(
            resp_view.permissions.len(),
            test_case.quantity_of_permissions
        );
    }
}
