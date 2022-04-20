use crate::common::{Config, Resources};
use crate::storage::postgres::user_repo::UserRepo;
use crate::usecases::user::errors::{SignError, UserUCError};
use crate::usecases::user::{crypto, get_user, user_creator};
use actix_web::http::header::Header;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_grants::proc_macro::has_any_role;
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use log::error;
use serde::Deserialize;
use web::Data;

#[get("users/{user_id}")]
#[has_any_role("AUTH_STAFF", "AUTH_MANAGER", "AUTH_ADMIN")]
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
#[has_any_role("AUTH_MANAGER", "AUTH_ADMIN")]
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

#[derive(Deserialize)]

pub struct UserCreationScheme {
    username: String,
    password: String,
}

#[post("users")]
#[has_any_role("AUTH_MANAGER", "AUTH_ADMIN")]
pub async fn create_user_handler(
    user_data: web::Json<UserCreationScheme>,
    resources: Data<Resources>,
) -> impl Responder {
    let username = user_data.username.to_string();
    let password = user_data.password.to_string();
    let user_access_model = UserRepo::new(resources.db_pool.clone());
    match user_creator::create_new_user(&user_access_model, username, password).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[post("users/sign_in")]
pub async fn sign_in_user_handler(
    req: HttpRequest,
    resources: Data<Resources>,
    config: Data<Config>,
) -> impl Responder {
    let auth_header = match Authorization::<Basic>::parse(&req) {
        Ok(header) => header,
        Err(_) => return HttpResponse::Forbidden().body("Forbidden"),
    };
    let username = auth_header.as_ref().user_id().to_string();
    let password = match auth_header.as_ref().password() {
        Some(cow_pass) => cow_pass.to_string(),
        None => return HttpResponse::Forbidden().body("Forbidden"),
    };
    let user_access_model = UserRepo::new(resources.db_pool.clone());
    match crypto::sign_in(
        &user_access_model,
        &config.security_config,
        username,
        password,
    )
    .await
    {
        Ok(signed_info) => HttpResponse::Ok().json(signed_info),
        Err(SignError::VerificationError) => HttpResponse::Forbidden().body("Forbidden"),
        Err(_) => {
            error!("Usecase fatal error during singin");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[derive(Deserialize)]

pub struct ValidateToken {
    jwt_token: String,
}

#[post("validate_jwt")]
pub async fn validate_jwt_handler(
    // req: HttpRequest,
    resources: Data<Resources>,
    config: Data<Config>,
    token_data: web::Json<ValidateToken>,
) -> impl Responder {
    // TODO authorization for srv methods
    // let auth_header = match Authorization::<Bearer>::parse(&req) {
    //     Ok(header) => header,
    //     Err(_) => return HttpResponse::Forbidden().body("Forbidden"),
    // };
    let user_access_model = UserRepo::new(resources.db_pool.clone());
    match crypto::verificate_jwt_token_and_enrich_perms(
        &user_access_model,
        &config.security_config,
        &token_data.jwt_token,
    )
    .await
    {
        Ok(perms) => HttpResponse::Ok().json(perms),
        Err(SignError::VerificationError) => HttpResponse::NotFound().body("not found"),
        Err(_) => {
            error!("Usecase fatal error during token validation");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}
