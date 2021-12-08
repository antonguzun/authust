use crate::usecases::user::entities::{User, UserContent};
use async_trait::async_trait;

pub enum RepoError {
    RepoFatalError,
    RepoTemporaryError,
    RepoNotFoundError,
}

pub enum UserUCError {
    FatalError,
    TemporaryError,
    NotFoundError,
}

#[async_trait]
pub trait FindUserById {
    async fn find_user_by_id(&self, user_id: i32) -> Result<User, RepoError>;
}

#[async_trait]
pub trait RemoveUserById {
    async fn remove_user_by_id(&self, user_id: i32) -> Result<(), RepoError>;
}

#[async_trait]
pub trait CreateUser {
    async fn create_user(&self, user: UserContent) -> Result<User, RepoError>;
}

#[async_trait]
pub trait GetUsers {
    async fn update_user_by_id(&self, limit: i64, offset: i64) -> Result<User, RepoError>;
}

pub async fn get_user_by_id(
    user_repo: &impl FindUserById,
    user_id: i32,
) -> Result<User, UserUCError> {
    match user_repo.find_user_by_id(user_id).await {
        Ok(user) => Ok(user),
        Err(RepoError::RepoNotFoundError) => Err(UserUCError::NotFoundError),
        Err(RepoError::RepoTemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}

pub async fn remove_user_by_id(
    user_repo: &impl RemoveUserById,
    user_id: i32,
) -> Result<(), UserUCError> {
    match user_repo.remove_user_by_id(user_id).await {
        Ok(()) => Ok(()),
        Err(RepoError::RepoNotFoundError) => Err(UserUCError::NotFoundError),
        Err(RepoError::RepoTemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}

pub async fn create_user(
    user_repo: &impl CreateUser,
    user: UserContent,
) -> Result<User, UserUCError> {
    match user_repo.create_user(user).await {
        Ok(user) => Ok(user),
        Err(RepoError::RepoTemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}
