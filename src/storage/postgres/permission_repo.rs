use crate::usecases::permission::entities::{Permission, PermissionForCreation};
use crate::usecases::permission::permission_creator::CreatePermission;
use crate::usecases::permission::permission_disabler::DisablePermission;
use crate::usecases::permission::permission_get_item::GetPermission;
use crate::usecases::user::errors::AccessModelError;
use async_trait::async_trait;
use chrono;
use deadpool_postgres::{Client, Pool};
use log::error;
use tokio_postgres::{Row, Statement};

pub struct PermissionRepo {
    db_pool: Pool,
}

impl PermissionRepo {
    pub fn new(db_pool: Pool) -> PermissionRepo {
        PermissionRepo { db_pool }
    }
}

const GET_BY_ID_QUERY: &str =
    "SELECT permission_id, permission_name, created_at, updated_at, is_deleted 
                                FROM permissions 
                                WHERE permission_id=$1";
const INSERT_PERMISSION_QUERY: &str = "INSERT INTO permissions (permission_name, created_at, updated_at, is_deleted) VALUES ($1, $2, $3, $4) RETURNING permission_id, permission_name, created_at, updated_at, is_deleted";
const DISABLE_PERMISSION_BY_ID_QUERY: &str = "UPDATE permissions SET is_deleted=TRUE, updated_at=$1 WHERE permission_id=$2 AND is_deleted=FALSE";

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

impl Permission {
    fn from_sql_result(row: &Row) -> Permission {
        Permission::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}
#[async_trait]
impl CreatePermission for PermissionRepo {
    async fn save_permission_in_storage(
        &self,
        perm_data: PermissionForCreation,
    ) -> Result<Permission, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, INSERT_PERMISSION_QUERY).await?;
        let now = chrono::Utc::now();
        match client
            .query(&stmt, &[&perm_data.permission_name, &now, &now, &false])
            .await
        {
            Ok(rows) if rows.len() == 1 => Ok(Permission::from_sql_result(&rows[0])),
            Ok(_) => {
                error!("During creation permission got count of retirning rows not equals one");
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
impl GetPermission for PermissionRepo {
    async fn get_permission_by_id(
        &self,
        permission_id: i32,
    ) -> Result<Permission, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, GET_BY_ID_QUERY).await?;
        match client.query(&stmt, &[&permission_id]).await {
            Ok(rows) if rows.len() == 1 => Ok(Permission::from_sql_result(&rows[0])),
            Ok(_) => {
                error!("During creation permission got count of retirning rows not equals one");
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
impl DisablePermission for PermissionRepo {
    async fn disable_permission_in_storage(
        &self,
        permission_id: i32,
    ) -> Result<(), AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, DISABLE_PERMISSION_BY_ID_QUERY).await?;

        let now = chrono::Utc::now();
        match client.execute(&stmt, &[&now, &permission_id]).await {
            Ok(res) if res != 0 => Ok(()),
            Ok(_) => Err(AccessModelError::NotFoundError),
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}
