use crate::common::Resources;
use crate::handlers::api::permissions::views::PermissionView;
use crate::storage::postgres::permission_repo::PermissionRepo;
use crate::usecases::permission::entities::PermissionForCreation;
use crate::usecases::permission::permission_creator::create_new_permission;
use crate::usecases::permission::permission_disabler::disable_permission_by_id;
use crate::usecases::permission::permission_get_item::get_permission_by_id;
use crate::usecases::user::errors::UserUCError;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::error;

#[post("permissions/")]
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

#[get("permissions/{perm_id}/")]
pub async fn get_permission_handler(
    permission_id: web::Path<i32>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let permission_access_model = PermissionRepo::new(resources.db_pool.clone());
    match get_permission_by_id(&permission_access_model, permission_id.into_inner()).await {
        Ok(permission) => HttpResponse::Ok().json(PermissionView::new(permission)),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[delete("permissions/{perm_id}/")]
pub async fn disable_permission_handler(
    permission_id: web::Path<i32>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let permission_access_model = PermissionRepo::new(resources.db_pool.clone());
    match disable_permission_by_id(&permission_access_model, permission_id.into_inner()).await {
        Ok(_) | Err(UserUCError::NotFoundError) => HttpResponse::NoContent().body(""),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}
