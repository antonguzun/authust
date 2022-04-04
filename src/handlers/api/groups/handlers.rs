use crate::common::Resources;
use crate::handlers::api::groups::views::GroupView;
use crate::storage::postgres::group_repo::GroupRepo;
use crate::usecases::group::group_get_item::get_group_by_id;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::error;

#[get("groups/{group_id}/")]
pub async fn get_group_handler(
    group_id: web::Path<i32>,
    resources: web::Data<Resources>,
) -> impl Responder {
    let group_access_model = GroupRepo::new(resources.db_pool.clone());
    // let group = get_group_by_id(group_access_model, group_id).await;
    match get_group_by_id(&group_access_model, group_id.into_inner()).await {
        Ok(group) => HttpResponse::Ok().json(GroupView::new(group)),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}
