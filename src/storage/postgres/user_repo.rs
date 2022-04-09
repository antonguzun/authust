use crate::storage::postgres::base::{get_client, prepare_stmt};
use crate::usecases::base_entities::AccessModelError;
use crate::usecases::user::crypto::SignInVerification;
use crate::usecases::user::entities::{User, UserForCreation};
use crate::usecases::user::get_user::{FindUserById, RemoveUserById};
use crate::usecases::user::user_creator::CreateUser;
use async_trait::async_trait;
use chrono;
use deadpool_postgres::Pool;
use log::error;
use tokio_postgres::Row;

pub struct UserRepo {
    db_pool: Pool,
}

impl UserRepo {
    pub fn new(db_pool: Pool) -> UserRepo {
        UserRepo { db_pool }
    }
}

const GET_BY_ID_QUERY: &str = "SELECT user_id, username, enabled, created_at, updated_at 
                                FROM users 
                                WHERE user_id=$1 AND is_deleted=FALSE";
const DELETE_BY_ID_QUERY: &str =
    "UPDATE users SET is_deleted=TRUE, updated_at=$1 WHERE user_id=$2 AND is_deleted=FALSE";
const INSERT_USER_QUERY: &str = "INSERT INTO users 
                                (username, password_hash, enabled, created_at, updated_at, is_deleted)
                                VALUES ($1, $2, $3, $4, $5, $6) 
                                RETURNING user_id, username, enabled, created_at, updated_at";
const FIND_USER_BY_VERIFICATION: &str =
    "SELECT user_id FROM users WHERE username=$1 AND password_hash=$2 AND is_deleted=FALSE";

impl User {
    fn from_sql_result(row: &Row) -> User {
        User::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}
#[async_trait]
impl FindUserById for UserRepo {
    async fn find_user_by_id(&self, user_id: i32) -> Result<User, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, GET_BY_ID_QUERY).await?;

        let rows = match client.query(&stmt, &[&user_id]).await {
            Ok(rows) => rows,
            Err(e) => {
                error!("{}", e);
                return Err(AccessModelError::FatalError);
            }
        };
        match rows.len() {
            0 => Err(AccessModelError::NotFoundError),
            _ => Ok(User::from_sql_result(&rows[0])),
        }
    }
}

#[async_trait]
impl RemoveUserById for UserRepo {
    async fn remove_user_by_id(&self, user_id: i32) -> Result<(), AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, DELETE_BY_ID_QUERY).await?;

        let now = chrono::Utc::now();
        match client.execute(&stmt, &[&now, &user_id]).await {
            Ok(res) if res != 0 => Ok(()),
            Ok(_) => Err(AccessModelError::NotFoundError),
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}

#[async_trait]
impl CreateUser for UserRepo {
    async fn save_user_in_storage(&self, user: UserForCreation) -> Result<User, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, INSERT_USER_QUERY).await?;

        let now = chrono::Utc::now();
        match client
            .query(
                &stmt,
                &[
                    &user.username,
                    &user.password_hash,
                    &true,
                    &now,
                    &now,
                    &false,
                ],
            )
            .await
        {
            Ok(rows) if rows.len() == 1 => Ok(User::from_sql_result(&rows[0])),
            Ok(_) => {
                error!("During creation user got count of retirning rows not equals one");
                Err(AccessModelError::FatalError)
            }
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}

#[async_trait]
impl SignInVerification for UserRepo {
    async fn verificate_default(
        &self,
        username: &str,
        hash: &str,
    ) -> Result<i32, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, FIND_USER_BY_VERIFICATION).await?;

        match client.query(&stmt, &[&username, &hash]).await {
            Ok(rows) if rows.len() != 0 => Ok(rows[0].get(0)),
            Ok(_) => Err(AccessModelError::NotFoundError),
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}
