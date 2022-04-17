use crate::common::Resources;
use crate::handlers::api::permissions::views::{PermissionListingView, PermissionView};
use crate::storage::postgres::permission_repo::PermissionRepo;
use crate::usecases::permission::entities::{PermissionForCreation, PermissionsFilters};
use crate::usecases::permission::errors::PermissionUCError;
use crate::usecases::permission::permission_creator::create_new_permission;
use crate::usecases::permission::permission_disabler::disable_permission_by_id;
use crate::usecases::permission::permission_get_item::get_permission_by_id;
use crate::usecases::permission::permission_get_list::get_permissions_by_filters;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use actix_web_grants::proc_macro::has_any_role;
use log::error;

#[get("permissions/{perm_id}")]
#[has_any_role("AUTH_STAFF", "AUTH_MANAGER", "AUTH_ADMIN")]
pub async fn get_permission_handler(
    permission_id: web::Path<i32>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let permission_access_model = PermissionRepo::new(resources.db_pool.clone());
    match get_permission_by_id(&permission_access_model, permission_id.into_inner()).await {
        Ok(permission) => HttpResponse::Ok().json(PermissionView::new(permission)),
        Err(PermissionUCError::NotFoundError) => HttpResponse::NotFound().body("Not Found"),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[get("permissions")]
#[has_any_role("AUTH_STAFF", "AUTH_MANAGER", "AUTH_ADMIN")]
pub async fn permissions_listing_handler(
    filters: web::Query<PermissionsFilters>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let permission_access_model = PermissionRepo::new(resources.db_pool.clone());
    match get_permissions_by_filters(&permission_access_model, filters.into_inner()).await {
        Ok(permissions) => HttpResponse::Ok().json(PermissionListingView::new(permissions)),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[post("permissions")]
#[has_any_role("AUTH_ADMIN")]
pub async fn create_permission_handler(
    perm_data: web::Json<PermissionForCreation>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let permission_access_model = PermissionRepo::new(resources.db_pool.clone());
    match create_new_permission(&permission_access_model, perm_data.into_inner()).await {
        Ok(permission) => HttpResponse::Created().json(PermissionView::new(permission)),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[delete("permissions/{perm_id}")]
#[has_any_role("AUTH_ADMIN")]
pub async fn disable_permission_handler(
    permission_id: web::Path<i32>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let permission_access_model = PermissionRepo::new(resources.db_pool.clone());
    match disable_permission_by_id(&permission_access_model, permission_id.into_inner()).await {
        Ok(_) | Err(PermissionUCError::NotFoundError) => HttpResponse::NoContent().body(""),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}
