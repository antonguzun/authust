use crate::usecases::user::entities::{InputRawUser, SingnedInfo};
use crate::usecases::user::errors::{AccessModelError, SignError};
use async_trait::async_trait;
use log::error;

use argon2::{self, Config};
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

#[async_trait]
pub trait SignInVerification {
    async fn verificate_default(&self, username: &str, hash: &str)
        -> Result<i32, AccessModelError>;
}

pub async fn sign_in(
    verificator: &impl SignInVerification,
    user_info: InputRawUser,
) -> Result<SingnedInfo, SignError> {
    let hash = match generate_hash(&user_info.password) {
        Ok(hash) => hash,
        Err(_) => return Err(SignError::FatalError),
    };
    match verificator
        .verificate_default(&user_info.username, &hash)
        .await
    {
        Ok(user_id) => Ok(SingnedInfo::new(user_id, "test_token".to_string())),
        Err(AccessModelError::NotFoundError) => Err(SignError::VerificationError),
        Err(AccessModelError::TemporaryError) => Err(SignError::TemporaryError),
        Err(_) => Err(SignError::FatalError),
    }
}
