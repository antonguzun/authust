use crate::storage::postgres::base::{delete_item, get_item, insert_item, SqlSerializer};
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
use tokio_postgres::types::ToSql;
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
const FIND_USER_BY_VERIFICATION: &str = "SELECT user_id 
    FROM users 
    WHERE username=$1 AND password_hash=$2 AND is_deleted=FALSE";

impl SqlSerializer<User> for User {
    fn from_sql_result(row: &Row) -> User {
        User::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}
#[async_trait]
impl FindUserById for UserRepo {
    async fn find_user_by_id(&self, user_id: i32) -> Result<User, AccessModelError> {
        get_item(&self.db_pool, GET_BY_ID_QUERY, &[&user_id]).await
    }
}

#[async_trait]
impl RemoveUserById for UserRepo {
    async fn remove_user_by_id(&self, user_id: i32) -> Result<(), AccessModelError> {
        let now = chrono::Utc::now();
        delete_item(&self.db_pool, DELETE_BY_ID_QUERY, &[&now, &user_id]).await
    }
}

#[async_trait]
impl CreateUser for UserRepo {
    async fn save_user_in_storage(&self, user: UserForCreation) -> Result<User, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[
            &user.username,
            &user.password_hash,
            &true,
            &now,
            &now,
            &false,
        ];
        insert_item(&self.db_pool, INSERT_USER_QUERY, params).await
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
