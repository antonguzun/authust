use crate::handlers::api::groups::views::GroupView;
use crate::storage::postgres::group_repo::GroupRepo;
use crate::usecases::group::entities::GroupForCreation;
use crate::usecases::group::group_creator::create_new_group;
use crate::usecases::group::group_disabler::disable_group_by_id;
use crate::usecases::group::group_get_item::get_group_by_id;
use crate::{common::Resources, usecases::group::errors::GroupUCError};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::error;

#[get("groups/{group_id}/")]
pub async fn get_group_handler(
    group_id: web::Path<i32>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let group_access_model = GroupRepo::new(resources.db_pool.clone());
    match get_group_by_id(&group_access_model, group_id.into_inner()).await {
        Ok(group) => HttpResponse::Ok().json(GroupView::new(group)),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[post("groups/")]
pub async fn create_group_handler(
    group_data: web::Json<GroupForCreation>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let group_access_model = GroupRepo::new(resources.db_pool.clone());
    match create_new_group(&group_access_model, group_data.into_inner()).await {
        Ok(group) => HttpResponse::Created().json(GroupView::new(group)),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[delete("groups/{group_id}/")]
pub async fn disable_group_handler(
    group_id: web::Path<i32>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let group_access_model = GroupRepo::new(resources.db_pool.clone());
    match disable_group_by_id(&group_access_model, group_id.into_inner()).await {
        Ok(_) | Err(GroupUCError::NotFoundError) => HttpResponse::NoContent().body(""),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}
