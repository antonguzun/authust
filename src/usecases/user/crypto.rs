use crate::common::SecurityConfig;
use crate::usecases::base_entities::AccessModelError;
use crate::usecases::user::entities::{SingnedInfo, JWT};
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

pub fn generate_jwt(config: &SecurityConfig, user_id: i32) -> Result<String, SignError> {
    let key: Hmac<Sha256> = match Hmac::new_from_slice(config.secret_key.as_bytes()) {
        Ok(key) => key,
        Err(e) => {
            error!("Error during initiatilization secret key for jwt: {}", e);
            return Err(SignError::FatalError);
        }
    };
    let expired_at = chrono::Utc::now() + chrono::Duration::days(config.expired_jwt_days.into());
    let jwt = JWT::new(user_id, expired_at.to_rfc3339());
    let content = json!(jwt);
    match content.sign_with_key(&key) {
        Ok(jwt) => Ok(jwt),
        Err(e) => {
            error!("Error during generation jwt: {}", e);
            Err(SignError::FatalError)
        }
    }
}

pub fn verificate_jwt(config: &SecurityConfig, jwt_token: &str) -> Result<JWT, SignError> {
    let key: Hmac<Sha256> = match Hmac::new_from_slice(config.secret_key.as_bytes()) {
        Ok(key) => key,
        Err(e) => {
            error!("Error during initiatilization secret key for jwt: {}", e);
            return Err(SignError::FatalError);
        }
    };
    match jwt_token.verify_with_key(&key) {
        Ok(jwt) => Ok(jwt),
        Err(_) => Err(SignError::VerificationError),
    }
}

#[async_trait]
pub trait SignInVerification {
    async fn verificate_default(&self, username: &str, hash: &str)
        -> Result<i32, AccessModelError>;
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

    let token_str = match generate_jwt(security_config, user_id) {
        Ok(jwt) => jwt,
        Err(_) => return Err(SignError::FatalError),
    };
    Ok(SingnedInfo::new(user_id, token_str.to_string()))
}
