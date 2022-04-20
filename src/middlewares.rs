use crate::common::{Config, Resources};
use crate::storage::postgres::user_repo::UserRepo;
use crate::usecases::users::crypto;
use crate::usecases::users::errors::SignError;

use actix_web::dev::ServiceRequest;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Data;
use actix_web::Error;
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use log::error;

pub async fn bearer_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = match req.app_data::<Data<Config>>() {
        Some(data) => data,
        None => {
            error!("Not found config for bearer_validator");
            return Err(ErrorInternalServerError("internal error"));
        }
    };
    let resources = match req.app_data::<Data<Resources>>() {
        Some(data) => data,
        None => {
            error!("Not found resources for bearer_validator");
            return Err(ErrorInternalServerError("internal error"));
        }
    };
    let user_access_model = UserRepo::new(resources.db_pool.clone());
    let perms = match crypto::verificate_jwt_token_and_enrich_perms(
        &user_access_model,
        &config.security_config,
        credentials.token(),
    )
    .await
    {
        Ok(permissions) => permissions,
        Err(SignError::VerificationError) => {
            return Err(ErrorUnauthorized("Wrong token".to_string()))
        }
        Err(_) => {
            error!("Usecase fatal error during token checking");
            return Err(ErrorInternalServerError("internal error"));
        }
    };
    req.attach(perms);
    Ok(req)
}
