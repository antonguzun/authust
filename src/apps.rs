use crate::handlers::api::groups::handlers::{
    bind_member_with_group_handler, bind_permission_with_group_handler, create_group_handler,
    disable_group_handler, get_group_handler, unbind_member_with_group_handler,
    unbind_permission_with_group_handler,
};
use crate::handlers::api::permissions::handlers::{
    create_permission_handler, disable_permission_handler, get_permission_handler,
    permissions_listing_handler,
};
use crate::handlers::api::users::{
    create_user_handler, delete_user_by_id, get_user_by_id, sign_in_user_handler,
};
use crate::handlers::system::handlers::{ping_handler, ready_handler};

use actix_web::web;

use crate::common::Config;
use crate::usecases::user::crypto::decode_jwt;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn bearer_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = Config::create_config();
    let claims = match decode_jwt(&config.security_config, credentials.token()) {
        Ok(claims) => claims,
        Err(_) => return Err(ErrorUnauthorized("Wrong token".to_string())),
    };
    req.attach(claims.permissions);
    Ok(req)
}

pub fn init_api_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_by_id)
        .service(create_user_handler)
        .service(delete_user_by_id)
        .service(get_permission_handler)
        .service(create_permission_handler)
        .service(disable_permission_handler)
        .service(permissions_listing_handler)
        .service(get_group_handler)
        .service(create_group_handler)
        .service(disable_group_handler)
        .service(bind_permission_with_group_handler)
        .service(unbind_permission_with_group_handler)
        .service(bind_member_with_group_handler)
        .service(unbind_member_with_group_handler);
}
pub fn init_external_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_by_id).service(sign_in_user_handler);
}

pub fn init_system(cfg: &mut web::ServiceConfig) {
    cfg.service(ready_handler).service(ping_handler);
}
