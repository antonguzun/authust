use crate::common::Resources;
use crate::usecases::user::get_user::{
    FindUserById, RemoveUserById, RepoError, SingleUserResult, User,
};
use async_trait::async_trait;
use deadpool_postgres::tokio_postgres::{Error, Row};
use deadpool_postgres::Pool;
use log::error;

pub struct UserRepo {
    db_pool: Pool,
}

impl UserRepo {
    pub fn new(db_pool: Pool) -> UserRepo {
        UserRepo { db_pool }
    }
}

#[async_trait]
impl FindUserById for UserRepo {
    async fn find_user_by_id(&self, user_id: i32) -> Result<User, RepoError> {
        let client = match self.db_pool.get().await {
            Ok(client) => client,
            Err(e) => {
                error!("{}", e);
                return Err(RepoError::RepoFatalError);
            }
        };

        let stmt = match client
            .prepare("SELECT user_id, name FROM users where user_id=$1")
            .await
        {
            Ok(stmt) => stmt,
            Err(e) => {
                error!("{}", e);
                return Err(RepoError::RepoFatalError);
            }
        };
        let rows = match client.query(&stmt, &[&user_id]).await {
            Ok(rows) => rows,
            Err(e) => {
                error!("{}", e);
                return Err(RepoError::RepoFatalError);
            }
        };
        match rows.len() {
            0 => Err(RepoError::RepoNotFoundError),
            _ => Ok(User::new(user_id, rows[0].get(1))),
        }
    }
}

#[async_trait]
impl RemoveUserById for UserRepo {
    async fn remove_user_by_id(&self, user_id: i32) -> Result<(), RepoError> {
        let client = match self.db_pool.get().await {
            Ok(client) => client,
            Err(e) => {
                error!("{}", e);
                return Err(RepoError::RepoFatalError);
            }
        };
        let stmt = match client.prepare("DELETE FROM users where user_id=$1").await {
            Ok(stmt) => stmt,
            Err(e) => {
                error!("{}", e);
                return Err(RepoError::RepoFatalError);
            }
        };
        let res = match client.execute(&stmt, &[&user_id]).await {
            Ok(rows) => rows,
            Err(e) => {
                error!("{}", e);
                return Err(RepoError::RepoFatalError);
            }
        };
        match res {
            0 => Err(RepoError::RepoNotFoundError),
            _ => Ok(()),
        }
    }
}
