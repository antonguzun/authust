use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> User {
        User { id, name }
    }
}

pub enum RepoError {
    RepoFatalError,
    RepoTemporaryError,
    RepoNotFoundError,
    RepoUnexpectedError,
}

pub enum UserUCError {
    FatalError,
    TemporaryError,
    NotFoundError,
}

type ServiceResult<T> = Result<T, UserUCError>;
pub type SingleUserResult = ServiceResult<User>;
pub type UserIdResult = ServiceResult<i32>;
pub type EmptyResult = ServiceResult<()>;

#[async_trait]
pub trait FindUserById {
    async fn find_user_by_id(&self, user_id: i32) -> Result<User, RepoError>;
}

pub async fn get_user_by_id(
    user_repo: &impl FindUserById,
    user_id: i32,
) -> Result<User, UserUCError> {
    let res = user_repo.find_user_by_id(user_id).await;
    match res {
        Ok(user) => Ok(user),
        Err(RepoError::RepoNotFoundError) => Err(UserUCError::NotFoundError),
        Err(_) => Err(UserUCError::FatalError),
    }
}

#[async_trait]
pub trait RemoveUserById {
    async fn remove_user_by_id(&self, user_id: i32) -> Result<(), RepoError>;
}

pub async fn remove_user_by_id(
    user_repo: &impl RemoveUserById,
    user_id: i32,
) -> Result<(), UserUCError> {
    let res = user_repo.remove_user_by_id(user_id).await;
    match res {
        Ok(()) => Ok(()),
        Err(RepoError::RepoNotFoundError) => Err(UserUCError::NotFoundError),
        Err(_) => Err(UserUCError::FatalError),
    }
}
