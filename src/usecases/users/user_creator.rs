use crate::usecases::base_entities::AccessModelError;
use crate::usecases::users::crypto::generate_hash;
use crate::usecases::users::entities::{User, UserForCreation};
use crate::usecases::users::errors::UserUCError;

use async_trait::async_trait;

#[async_trait]
pub trait CreateUser {
    async fn save_user_in_storage(&self, user: UserForCreation) -> Result<User, AccessModelError>;
}

pub async fn create_new_user(
    user_access_model: &impl CreateUser,
    username: String,
    password: String,
) -> Result<User, UserUCError> {
    let hash = match generate_hash(&password) {
        Ok(hash) => hash,
        Err(_) => return Err(UserUCError::FatalError),
    };
    let user_data = UserForCreation {
        username: username,
        password_hash: hash,
    };
    match user_access_model.save_user_in_storage(user_data).await {
        Ok(user) => Ok(user),
        Err(AccessModelError::AlreadyExists) => Err(UserUCError::AlreadyExists),
        Err(AccessModelError::TemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}
