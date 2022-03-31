use crate::usecases::user::crypto::SignInVerification;
use crate::usecases::user::entities::{User, UserForCreation};
use crate::usecases::user::errors::AccessModelError;
use crate::usecases::user::get_user::{FindUserById, RemoveUserById};
use crate::usecases::user::user_creator::CreateUser;
use async_trait::async_trait;
use chrono;
use deadpool_postgres::{Client, Pool};
use log::error;
use tokio_postgres::{Row, Statement};

const READY_QUERY: &str = "SELECT 1";

async fn get_client(db_pool: &Pool) -> Result<Client, AccessModelError> {
    match db_pool.get().await {
        Ok(client) => Ok(client),
        Err(e) => {
            error!("{}", e);
            Err(AccessModelError::TemporaryError)
        }
    }
}

async fn prepare_stmt(client: &Client, query: &str) -> Result<Statement, AccessModelError> {
    match client.prepare(query).await {
        Ok(stmt) => Ok(stmt),
        Err(e) => {
            error!("{}", e);
            Err(AccessModelError::FatalError)
        }
    }
}

pub async fn check_db(db_pool: &Pool) -> Result<(), AccessModelError> {
    let client = get_client(db_pool).await?;
    let stmt = prepare_stmt(&client, READY_QUERY).await?;
    match client.query(&stmt, &[]).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{}", e);
            Err(AccessModelError::FatalError)
        }
    }
}
