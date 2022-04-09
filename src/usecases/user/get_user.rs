use crate::usecases::base_entities::AccessModelError;
use crate::usecases::user::entities::User;
use crate::usecases::user::errors::UserUCError;
use async_trait::async_trait;

#[async_trait]
pub trait FindUserById {
    async fn find_user_by_id(&self, user_id: i32) -> Result<User, AccessModelError>;
}

#[async_trait]
pub trait RemoveUserById {
    async fn remove_user_by_id(&self, user_id: i32) -> Result<(), AccessModelError>;
}

#[async_trait]
pub trait GetUsers {
    async fn update_user_by_id(&self, limit: i64, offset: i64) -> Result<User, UserUCError>;
}

pub async fn get_user_by_id(
    user_repo: &impl FindUserById,
    user_id: i32,
) -> Result<User, UserUCError> {
    match user_repo.find_user_by_id(user_id).await {
        Ok(user) => Ok(user),
        Err(AccessModelError::NotFoundError) => Err(UserUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}

pub async fn remove_user_by_id(
    user_repo: &impl RemoveUserById,
    user_id: i32,
) -> Result<(), UserUCError> {
    match user_repo.remove_user_by_id(user_id).await {
        Ok(()) => Ok(()),
        Err(AccessModelError::NotFoundError) => Err(UserUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}
