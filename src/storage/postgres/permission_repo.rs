use crate::usecases::permission::entities::{
    Permission, PermissionForCreation, PermissionsFilters,
};
use crate::usecases::permission::errors::AccessModelError;
use crate::usecases::permission::permission_creator::CreatePermission;
use crate::usecases::permission::permission_disabler::DisablePermission;
use crate::usecases::permission::permission_get_item::GetPermission;
use crate::usecases::permission::permission_get_list::GetPermissionsList;
use async_trait::async_trait;
use chrono;
use deadpool_postgres::{Client, Pool};
use log::error;
use tokio_postgres::types::ToSql;
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
const INSERT_PERMISSION_QUERY: &str =
    "INSERT INTO permissions (permission_name, created_at, updated_at, is_deleted) 
    VALUES ($1, $2, $3, $4) 
    RETURNING permission_id, permission_name, created_at, updated_at, is_deleted";
const DISABLE_PERMISSION_BY_ID_QUERY: &str = "UPDATE permissions 
    SET is_deleted=TRUE, updated_at=$1 
    WHERE permission_id=$2 AND is_deleted=FALSE";
const GET_BY_FILTERS_QUERY: &str =
    "SELECT permission_id, permission_name, created_at, updated_at, is_deleted 
    FROM permissions 
    WHERE TRUE";

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

fn build_listing_query(base_query: &str, filters: &PermissionsFilters) -> String {
    let mut query = base_query.to_string();
    let mut cnt: u8 = 1;
    match filters.permission_id {
        Some(_) => {
            query.push_str(&format!(" AND permission_id=${}", cnt));
            cnt += 1;
        }
        None => (),
    }
    match filters.permission_name {
        Some(_) => {
            query.push_str(&format!(" AND permission_name=${}", cnt));
            cnt += 1;
        }
        None => (),
    }
    match filters.group_id {
        Some(_) => {
            query.push_str(&format!(" AND group_id=${}", cnt));
            cnt += 1;
        }
        None => (),
    }
    match filters.is_deleted {
        Some(_) => {
            query.push_str(&format!(" AND is_deleted=${}", cnt));
            cnt += 1;
        }
        None => (),
    }
    match filters.offset {
        Some(_) => {
            query.push_str(&format!(" OFFSET ${}", cnt));
            cnt += 1;
        }
        None => query.push_str(" OFFSET 0"),
    }
    match filters.limit {
        Some(_) => query.push_str(&format!(" LIMIT ${}", cnt)),
        None => query.push_str(" LIMIT 100"),
    }
    query
}

fn build_listing_params(filters: &PermissionsFilters) -> Vec<&(dyn ToSql + Sync)> {
    let mut params: Vec<&(dyn ToSql + Sync)> = vec![];
    match &filters.permission_id {
        Some(permission_id) => params.push(permission_id),
        None => (),
    }
    match &filters.permission_name {
        Some(permission_name) => params.push(permission_name),
        None => (),
    }
    match &filters.group_id {
        Some(group_id) => params.push(group_id),
        None => (),
    }
    match &filters.is_deleted {
        Some(is_deleted) => params.push(is_deleted),
        None => (),
    }
    match &filters.offset {
        Some(offset) => params.push(offset),
        None => (),
    }
    match &filters.limit {
        Some(limit) => params.push(limit),
        None => (),
    }
    params
}

#[async_trait]
impl GetPermissionsList for PermissionRepo {
    async fn get_permissions_by_filters(
        &self,
        filters: PermissionsFilters,
    ) -> Result<Vec<Permission>, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let query = build_listing_query(GET_BY_FILTERS_QUERY, &filters);
        let params = build_listing_params(&filters);
        let stmt = prepare_stmt(&client, &query).await?;
        match client.query(&stmt, &params).await {
            Ok(rows) => Ok(rows
                .into_iter()
                .map(|row| Permission::from_sql_result(&row))
                .collect()),
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}
