use crate::storage::postgres::base::{
    delete_item, get_client, get_count, get_item, get_list, insert_item,
};
use crate::storage::postgres::base::{CountQueryBuilder, ListingQueryBuilder, SqlSerializer};

use crate::usecases::base_entities::AccessModelError;
use crate::usecases::permission::entities::{
    Permission, PermissionForCreation, PermissionsFilters, PermissionsList,
};
use crate::usecases::permission::permission_creator::CreatePermission;
use crate::usecases::permission::permission_disabler::DisablePermission;
use crate::usecases::permission::permission_get_item::GetPermission;
use crate::usecases::permission::permission_get_list::GetPermissionsList;

use async_trait::async_trait;
use chrono;
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;

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
    "SELECT permission_id, permission_name, p.created_at, p.updated_at, p.is_deleted 
    FROM permissions p";
const GET_TOTAL_BY_FILTERS_QUERY: &str = "SELECT count(1) FROM permissions p";

impl SqlSerializer<Permission> for Permission {
    fn from_sql_result(row: &Row) -> Permission {
        Permission::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}
#[async_trait]
impl GetPermission for PermissionRepo {
    async fn get_permission_by_id(&self, perm_id: i32) -> Result<Permission, AccessModelError> {
        get_item(&self.db_pool, GET_BY_ID_QUERY, &[&perm_id]).await
    }
}
#[async_trait]
impl CreatePermission for PermissionRepo {
    async fn save_permission_in_storage(
        &self,
        perm_data: PermissionForCreation,
    ) -> Result<Permission, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&perm_data.permission_name, &now, &now, &false];
        insert_item(&self.db_pool, INSERT_PERMISSION_QUERY, params).await
    }
}

#[async_trait]
impl DisablePermission for PermissionRepo {
    async fn disable_permission_in_storage(
        &self,
        permission_id: i32,
    ) -> Result<(), AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &permission_id];
        delete_item(&self.db_pool, DISABLE_PERMISSION_BY_ID_QUERY, params).await
    }
}

fn add_permission_filters<'r, 'a>(
    query: &'r mut String,
    filters: &'a PermissionsFilters,
) -> (&'r mut String, Vec<&'a (dyn ToSql + Sync)>) {
    let mut params: Vec<&(dyn ToSql + Sync)> = vec![];
    let mut cnt: u8 = 1;
    match &filters.role_id {
        Some(role_id) => {
            query.push_str(&format!(
                " LEFT JOIN role_permissions USING(permission_id) WHERE role_id=${}",
                cnt
            ));
            cnt += 1;
            params.push(role_id)
        }
        None => query.push_str(" WHERE TRUE"),
    }
    match &filters.permission_id {
        Some(permission_id) => {
            params.push(permission_id);
            query.push_str(&format!(" AND p.permission_id=${}", cnt));
            cnt += 1;
        }
        None => (),
    }
    match &filters.permission_name {
        Some(permission_name) => {
            params.push(permission_name);
            query.push_str(&format!(" AND permission_name=${}", cnt));
            cnt += 1;
        }
        None => (),
    }
    match &filters.is_deleted {
        Some(is_deleted) => {
            params.push(is_deleted);
            query.push_str(&format!(" AND p.is_deleted=${}", cnt));
        }
        None => (),
    }
    (query, params)
}

impl ListingQueryBuilder for PermissionsFilters {
    fn build_listing_query_with_params(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut query = GET_BY_FILTERS_QUERY.to_string();
        let (query, params) = add_permission_filters(&mut query, &self);
        query.push_str(&format!(" ORDER BY permission_id DESC"));
        query.push_str(&format!(" OFFSET {}", &self.offset));
        query.push_str(&format!(" LIMIT {}", &self.limit));
        (query.to_string(), params)
    }
}
impl CountQueryBuilder for PermissionsFilters {
    fn build_count_query_with_params(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut query = GET_TOTAL_BY_FILTERS_QUERY.to_string();
        let (query, params) = add_permission_filters(&mut query, &self);
        (query.to_string(), params)
    }
}

#[async_trait]
impl GetPermissionsList for PermissionRepo {
    async fn get_permissions_by_filters(
        &self,
        filters: PermissionsFilters,
    ) -> Result<PermissionsList, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let perms = get_list(&client, filters.clone()).await?;
        let total = get_count(&client, filters).await?;
        Ok(PermissionsList::new(perms, total))
    }
}
