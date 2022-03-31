use crate::usecases::user::entities::{InputRawUser, SingnedInfo};
use crate::usecases::user::errors::{AccessModelError, SignError};
use argon2::{self, Config};
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use log::error;
use sha2::Sha256;
use std::collections::BTreeMap;

static SALT: &[u8] = "22f65e79-496a-4b48-8abc-f83e1e52aa4e".as_bytes();

pub fn generate_hash(password: &str) -> Result<String, &str> {
    let config = Config::default();
    match argon2::hash_encoded(password.as_bytes(), SALT, &config) {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => {
            error!("hashing password error: {}", e);
            Err("hashing password error")
        }
    }
}

pub fn generate_jwt(secret_key: &String, user_id: i32) -> Result<String, &str> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("user_id", user_id);
    match claims.sign_with_key(&key) {
        Ok(jwt) => Ok(jwt),
        Err(e) => {
            error!("Error during generation jwt {}", e);
            Err("jwt error")
        }
    }
}

#[async_trait]
pub trait SignInVerification {
    async fn verificate_default(&self, username: &str, hash: &str)
        -> Result<i32, AccessModelError>;
}

pub async fn sign_in(
    verificator: &impl SignInVerification,
    secret_key: &String,
    user_info: InputRawUser,
) -> Result<SingnedInfo, SignError> {
    let hash = match generate_hash(&user_info.password) {
        Ok(hash) => hash,
        Err(_) => return Err(SignError::FatalError),
    };
    let user_id = match verificator
        .verificate_default(&user_info.username, &hash)
        .await
    {
        Ok(user_id) => user_id,
        Err(AccessModelError::NotFoundError) => return Err(SignError::VerificationError),
        Err(AccessModelError::TemporaryError) => return Err(SignError::TemporaryError),
        Err(_) => return Err(SignError::FatalError),
    };

    let token_str = match generate_jwt(secret_key, user_id) {
        Ok(jwt) => jwt,
        Err(_) => return Err(SignError::FatalError),
    };
    Ok(SingnedInfo::new(user_id, token_str.to_string()))
}
