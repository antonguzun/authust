use crate::common::SecurityConfig;
use crate::usecases::base_entities::AccessModelError;
use crate::usecases::user::entities::{Claims, SingnedInfo};
use crate::usecases::user::errors::SignError;
use argon2::{self, Config};
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use log::error;
use serde_json::json;
use sha2::Sha256;

static SALT: &[u8] = "22f65e79-496a-4b48-8abc-f83e1e52aa4e".as_bytes();

pub fn generate_hash(password: &str) -> Result<String, SignError> {
    let config = Config::default();
    match argon2::hash_encoded(password.as_bytes(), SALT, &config) {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => {
            error!("hashing password error: {}", e);
            Err(SignError::FatalError)
        }
    }
}

pub fn generate_jwt(
    config: &SecurityConfig,
    user_id: i32,
    groups: Vec<String>,
) -> Result<String, SignError> {
    let key: Hmac<Sha256> = match Hmac::new_from_slice(config.secret_key.as_bytes()) {
        Ok(key) => key,
        Err(e) => {
            error!("Error during initiatilization secret key for jwt: {}", e);
            return Err(SignError::FatalError);
        }
    };
    let claims = Claims::new(user_id, config.expired_jwt_days, groups);
    let content = json!(claims);
    match content.sign_with_key(&key) {
        Ok(jwt) => Ok(jwt),
        Err(e) => {
            error!("Error during generation jwt: {}", e);
            Err(SignError::FatalError)
        }
    }
}

pub fn decode_jwt(config: &SecurityConfig, jwt_token: &str) -> Result<Claims, SignError> {
    let key: Hmac<Sha256> = match Hmac::new_from_slice(config.secret_key.as_bytes()) {
        Ok(key) => key,
        Err(e) => {
            error!("Error during initiatilization secret key for jwt: {}", e);
            return Err(SignError::FatalError);
        }
    };
    match jwt_token.verify_with_key(&key) {
        Ok(claims) => Ok(claims),
        Err(_) => Err(SignError::VerificationError),
    }
}

#[async_trait]
pub trait SignInVerification {
    async fn verificate_default(&self, username: &str, hash: &str)
        -> Result<i32, AccessModelError>;
    async fn get_users_groups(&self, user_id: &i32) -> Result<Vec<String>, AccessModelError>;
    async fn get_users_perms(&self, user_id: &i32) -> Result<Vec<String>, AccessModelError>;
}

pub async fn sign_in(
    verificator: &impl SignInVerification,
    security_config: &SecurityConfig,
    username: String,
    password: String,
) -> Result<SingnedInfo, SignError> {
    let hash = match generate_hash(&password) {
        Ok(hash) => hash,
        Err(_) => return Err(SignError::FatalError),
    };
    let user_id = match verificator.verificate_default(&username, &hash).await {
        Ok(user_id) => user_id,
        Err(AccessModelError::NotFoundError) => return Err(SignError::VerificationError),
        Err(AccessModelError::TemporaryError) => return Err(SignError::TemporaryError),
        Err(_) => return Err(SignError::FatalError),
    };
    let groups = match verificator.get_users_groups(&user_id).await {
        Ok(groups) => groups,
        Err(_) => return Err(SignError::FatalError),
    };
    let token_str = match generate_jwt(security_config, user_id, groups) {
        Ok(jwt) => jwt,
        Err(_) => return Err(SignError::FatalError),
    };
    Ok(SingnedInfo::new(user_id, token_str.to_string()))
}

pub async fn verificate_jwt_token_and_enrich_perms(
    verificator: &impl SignInVerification,
    config: &SecurityConfig,
    jwt_token: &str,
) -> Result<Vec<String>, SignError> {
    let claims = decode_jwt(config, jwt_token)?;
    match verificator.get_users_perms(&claims.user_id).await {
        Ok(perms) => Ok(perms),
        Err(_) => return Err(SignError::FatalError),
    }
}
