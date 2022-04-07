use crate::common::{Config, Resources};
use crate::storage::postgres::user_repo::UserRepo;
use crate::usecases::user::entities::InputRawUser;
use crate::usecases::user::errors::{SignError, UserUCError};
use crate::usecases::user::{crypto, get_user, user_creator};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::error;
use web::Data;

#[get("users/{user_id}")]
pub async fn get_user_by_id(user_id: web::Path<u32>, resources: Data<Resources>) -> impl Responder {
    let user_id = user_id.into_inner() as i32;
    let user_repo = UserRepo::new(resources.db_pool.clone());
    match get_user::get_user_by_id(&user_repo, user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(UserUCError::NotFoundError) => HttpResponse::NotFound().body("Not Found"),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[delete("users/{user_id}")]
pub async fn delete_user_by_id(
    user_id: web::Path<u32>,
    resources: Data<Resources>,
) -> impl Responder {
    let user_id = user_id.into_inner() as i32;
    let user_repo = UserRepo::new(resources.db_pool.clone());
    match get_user::remove_user_by_id(&user_repo, user_id).await {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(UserUCError::NotFoundError) => HttpResponse::NoContent().body(""),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[post("users")]
pub async fn create_user_handler(
    user_data: web::Json<InputRawUser>,
    resources: Data<Resources>,
) -> impl Responder {
    let user_access_model = UserRepo::new(resources.db_pool.clone());
    match user_creator::create_new_user(&user_access_model, user_data.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[post("users/sign_in")]
pub async fn sign_in_user_handler(
    user_data: web::Json<InputRawUser>,
    resources: Data<Resources>,
    config: Data<Config>,
) -> impl Responder {
    let user_access_model = UserRepo::new(resources.db_pool.clone());
    match crypto::sign_in(
        &user_access_model,
        &config.security_config,
        user_data.into_inner(),
    )
    .await
    {
        Ok(signed_info) => HttpResponse::Ok().json(signed_info),
        Err(SignError::VerificationError) => HttpResponse::Forbidden().body("Forbidden"),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}
