use crate::common::Resources;
use crate::handlers::api::roles::views::{
    BindingMemberCreationScheme, BindingPermissionCreationScheme, MemberBindingQuery,
    PermissionBindingQuery, RoleMemberBindingView, RolePermissionBindingView, RoleView,
};
use crate::storage::postgres::role_repo::RoleRepo;
use crate::usecases::roles::entities::RoleForCreation;
use crate::usecases::roles::errors::RoleUCError;
use crate::usecases::roles::role_creator::create_new_role;
use crate::usecases::roles::role_disabler::disable_role_by_id;
use crate::usecases::roles::role_get_item::get_role_by_id;
use crate::usecases::roles::role_members_binder::{bind_member_to_role, unbind_member_to_role};
use crate::usecases::roles::role_permissions_binder::{
    bind_permission_to_role, unbind_permission_to_role,
};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web_grants::proc_macro::has_permissions;
use log::error;

#[get("roles/{role_id}")]
#[has_permissions("READ_ROLE")]
pub async fn get_role_handler(
    role_id: web::Path<i32>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let role_access_model = RoleRepo::new(resources.db_pool.clone());
    match get_role_by_id(&role_access_model, role_id.into_inner()).await {
        Ok(role) => HttpResponse::Ok().json(RoleView::new(role)),
        Err(RoleUCError::NotFoundError) => HttpResponse::NotFound().body("not found"),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[post("roles")]
#[has_permissions("WRITE_ROLE")]
pub async fn create_role_handler(
    role_data: web::Json<RoleForCreation>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let role_access_model = RoleRepo::new(resources.db_pool.clone());
    match create_new_role(&role_access_model, role_data.into_inner()).await {
        Ok(role) => HttpResponse::Created().json(RoleView::new(role)),
        Err(RoleUCError::AlreadyExists) => HttpResponse::BadRequest().body("already exists"),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[delete("roles/{role_id}")]
#[has_permissions("WRITE_ROLE")]
pub async fn disable_role_handler(
    role_id: web::Path<i32>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let role_access_model = RoleRepo::new(resources.db_pool.clone());
    match disable_role_by_id(&role_access_model, role_id.into_inner()).await {
        Ok(_) | Err(RoleUCError::NotFoundError) => HttpResponse::NoContent().body(""),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[put("roles/bind_permisson")]
#[has_permissions("BIND_ROLE_WITH_PERMISSION")]
pub async fn bind_permission_with_role_handler(
    data: web::Json<BindingPermissionCreationScheme>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let role_access_model = RoleRepo::new(resources.db_pool.clone());
    match bind_permission_to_role(&role_access_model, data.role_id, data.permission_id).await {
        Ok(binding) => HttpResponse::Ok().json(RolePermissionBindingView::new(binding)),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[put("roles/{role_id}/unbind_permisson/{permission_id}")]
#[has_permissions("BIND_ROLE_WITH_PERMISSION")]
pub async fn unbind_permission_with_role_handler(
    data: web::Path<PermissionBindingQuery>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let role_access_model = RoleRepo::new(resources.db_pool.clone());
    match unbind_permission_to_role(&role_access_model, data.role_id, data.permission_id).await {
        Ok(binding) => HttpResponse::Ok().json(RolePermissionBindingView::new(binding)),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[put("roles/bind_member")]
#[has_permissions("BIND_USER_WITH_ROLE")]
pub async fn bind_member_with_role_handler(
    data: web::Json<BindingMemberCreationScheme>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let role_access_model = RoleRepo::new(resources.db_pool.clone());
    match bind_member_to_role(&role_access_model, data.role_id, data.user_id).await {
        Ok(binding) => HttpResponse::Ok().json(RoleMemberBindingView::new(binding)),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[put("roles/{role_id}/unbind_member/{user_id}")]
#[has_permissions("BIND_USER_WITH_ROLE")]
pub async fn unbind_member_with_role_handler(
    data: web::Path<MemberBindingQuery>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let role_access_model = RoleRepo::new(resources.db_pool.clone());
    match unbind_member_to_role(&role_access_model, data.role_id, data.user_id).await {
        Ok(binding) => HttpResponse::Ok().json(RoleMemberBindingView::new(binding)),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}
