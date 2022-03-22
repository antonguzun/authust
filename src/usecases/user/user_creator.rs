use crate::usecases::user::entities::{InputRawUser, User, UserForCreation};
use async_trait::async_trait;

pub enum AccessModelError {
    FatalError,
    TemporaryError,
    NotFoundError,
    AlreadyExists,
}
pub enum UserUCError {
    FatalError,
    TemporaryError,
    NotFoundError,
    AlreadyExists,
}

#[async_trait]
pub trait CreateUser {
    async fn save_user_in_storage(&self, user: UserForCreation) -> Result<User, AccessModelError>;
}

pub async fn create_new_user(
    user_access_model: &impl CreateUser,
    raw_user: InputRawUser,
) -> Result<User, UserUCError> {
    let user_data = UserForCreation {
        username: raw_user.username,
        password_hash: (&"2345").to_string(),
    }; // TODO implement hashing
    match user_access_model.save_user_in_storage(user_data).await {
        Ok(user) => Ok(user),
        Err(AccessModelError::TemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}
