use crate::storage::postgres::base::{get_client, prepare_stmt};
use crate::usecases::base_entities::AccessModelError;
use crate::usecases::group::entities::{Group, GroupForCreation, GroupsPermissionBinding};
use crate::usecases::group::group_binder::GroupBindPermission;
use crate::usecases::group::group_creator::CreateGroup;
use crate::usecases::group::group_disabler::DisableGroup;
use crate::usecases::group::group_get_item::GetGroup;

use async_trait::async_trait;
use chrono;
use deadpool_postgres::Pool;
use log::error;
use tokio_postgres::{error::SqlState, Row};

pub struct GroupRepo {
    db_pool: Pool,
}

impl GroupRepo {
    pub fn new(db_pool: Pool) -> GroupRepo {
        GroupRepo { db_pool }
    }
}

const GET_GROUP_BY_ID_QUERY: &str =
    "SELECT group_id, group_name, created_at, updated_at, is_deleted 
    FROM groups 
    WHERE group_id=$1";
const INSERT_GROUP_QUERY: &str =
    "INSERT INTO groups (group_name, created_at, updated_at, is_deleted) 
    VALUES ($1, $2, $3, $4) 
    RETURNING group_id, group_name, created_at, updated_at, is_deleted";
const DISABLE_GROUP_BY_ID_QUERY: &str = "UPDATE groups 
    SET is_deleted=TRUE, updated_at=$1 
    WHERE group_id=$2 AND is_deleted=FALSE";

const GET_GROUPS_PERMISSION_BINDING_BY_PK_QUERY: &str =
    "SELECT permission_id, group_id, created_at, updated_at, is_deleted 
    FROM group_permissions 
    WHERE permission_id=$1 AND group_id=$2";
const ENABLE_GROUPS_PERMISSION_BINDING_QUERY: &str = "UPDATE group_permissions 
    SET is_deleted=FALSE, updated_at=$1 
    WHERE permission_id=$2 AND group_id=$3
    RETURNING permission_id, group_id, created_at, updated_at, is_deleted";
const ADD_PERMISSION_TO_GROUPS_QUERY: &str = "INSERT INTO group_permissions 
    (permission_id, group_id, created_at, updated_at, is_deleted)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING permission_id, group_id, created_at, updated_at, is_deleted";
#[warn(dead_code)]
const DISABLE_GROUPS_PERMISSION_BINDING_QUERY: &str = "UPDATE group_permissions 
    SET is_deleted=TRUE, updated_at=$1 
    WHERE permission_id=$2 AND group_id=$3
    RETURNING permission_id, group_id, created_at, updated_at, is_deleted";

impl Group {
    fn from_sql_result(row: &Row) -> Group {
        Group::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}
#[async_trait]
impl GetGroup for GroupRepo {
    async fn get_group_by_id(&self, group_id: i32) -> Result<Group, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, GET_GROUP_BY_ID_QUERY).await?;
        match client.query(&stmt, &[&group_id]).await {
            Ok(rows) if rows.len() == 1 => Ok(Group::from_sql_result(&rows[0])),
            Ok(rows) if rows.len() == 0 => Err(AccessModelError::NotFoundError),
            Ok(_) => {
                error!("During getting group count of retirning rows not equals one");
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
impl CreateGroup for GroupRepo {
    async fn save_group_in_storage(
        &self,
        group_data: GroupForCreation,
    ) -> Result<Group, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, INSERT_GROUP_QUERY).await?;
        let now = chrono::Utc::now();
        match client
            .query(&stmt, &[&group_data.group_name, &now, &now, &false])
            .await
        {
            Ok(rows) if rows.len() == 1 => Ok(Group::from_sql_result(&rows[0])),
            Ok(_) => {
                error!("During creation group got count of retirning rows not equals one");
                Err(AccessModelError::FatalError)
            }
            Err(e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION) => {
                Err(AccessModelError::AlreadyExists)
            }
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}

#[async_trait]
impl DisableGroup for GroupRepo {
    async fn disable_group_by_id(&self, group_id: i32) -> Result<(), AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, DISABLE_GROUP_BY_ID_QUERY).await?;

        let now = chrono::Utc::now();
        match client.execute(&stmt, &[&now, &group_id]).await {
            Ok(res) if res != 0 => Ok(()),
            Ok(_) => Err(AccessModelError::NotFoundError),
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}

impl GroupsPermissionBinding {
    fn from_sql_result(row: &Row) -> GroupsPermissionBinding {
        GroupsPermissionBinding::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}

#[async_trait]
impl GroupBindPermission for GroupRepo {
    async fn get_groups_permission_binding(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, GET_GROUPS_PERMISSION_BINDING_BY_PK_QUERY).await?;
        match client.query(&stmt, &[&perm_id, &group_id]).await {
            Ok(rows) if rows.len() == 1 => Ok(GroupsPermissionBinding::from_sql_result(&rows[0])),
            Ok(rows) if rows.len() == 0 => Err(AccessModelError::NotFoundError),
            Ok(_) => {
                error!("During getting binding count of retirning rows not equals one");
                Err(AccessModelError::FatalError)
            }
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
    async fn enable_existed_groups_permission_binding(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, ENABLE_GROUPS_PERMISSION_BINDING_QUERY).await?;
        let now = chrono::Utc::now();
        match client.query(&stmt, &[&now, &perm_id, &group_id]).await {
            Ok(rows) if rows.len() == 1 => Ok(GroupsPermissionBinding::from_sql_result(&rows[0])),
            Ok(rows) if rows.len() == 0 => Err(AccessModelError::NotFoundError),
            Ok(_) => {
                error!("During binding permission count of retirning rows not equals one");
                Err(AccessModelError::FatalError)
            }
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
    async fn add_permission_to_group(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, ADD_PERMISSION_TO_GROUPS_QUERY).await?;
        let now = chrono::Utc::now();
        match client
            .query(&stmt, &[&perm_id, &group_id, &now, &now, &false])
            .await
        {
            Ok(rows) if rows.len() == 1 => Ok(GroupsPermissionBinding::from_sql_result(&rows[0])),
            Ok(_) => {
                error!("During creation binding got count of retirning rows not equals one");
                Err(AccessModelError::FatalError)
            }
            Err(e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION) => {
                Err(AccessModelError::AlreadyExists)
            }
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
    async fn disable_existed_groups_permission_binding(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, DISABLE_GROUPS_PERMISSION_BINDING_QUERY).await?;

        let now = chrono::Utc::now();
        match client.query(&stmt, &[&now, &perm_id, &group_id]).await {
            Ok(rows) if rows.len() == 1 => Ok(GroupsPermissionBinding::from_sql_result(&rows[0])),
            Ok(rows) if rows.len() == 0 => Err(AccessModelError::NotFoundError),
            Ok(_) => {
                error!("During unbinding permission count of retirning rows not equals one");
                Err(AccessModelError::FatalError)
            }
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}
