use crate::usecases::base_entities::AccessModelError;
use crate::usecases::user::crypto::generate_hash;
use crate::usecases::user::entities::{InputRawUser, User, UserForCreation};
use crate::usecases::user::errors::UserUCError;

use async_trait::async_trait;

#[async_trait]
pub trait CreateUser {
    async fn save_user_in_storage(&self, user: UserForCreation) -> Result<User, AccessModelError>;
}

pub async fn create_new_user(
    user_access_model: &impl CreateUser,
    raw_user: InputRawUser,
) -> Result<User, UserUCError> {
    let hash = match generate_hash(&raw_user.password) {
        Ok(hash) => hash,
        Err(_) => return Err(UserUCError::FatalError),
    };
    let user_data = UserForCreation {
        username: raw_user.username,
        password_hash: hash,
    };
    match user_access_model.save_user_in_storage(user_data).await {
        Ok(user) => Ok(user),
        Err(AccessModelError::TemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}
