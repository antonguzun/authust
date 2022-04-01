use crate::common::Resources;
use crate::storage::postgres::system::check_db;
use actix_web::{get, web, HttpResponse, Responder};

#[get("ping/")]
pub async fn ping_handler() -> impl Responder {
    HttpResponse::Ok().body("success")
}

#[get("ready/")]
pub async fn ready_handler(resources: web::Data<Resources>) -> impl Responder {
    match check_db(&resources.db_pool).await {
        Ok(_) => HttpResponse::Ok().body("success"),
        _ => HttpResponse::InternalServerError().body("internal error"),
    }
}
