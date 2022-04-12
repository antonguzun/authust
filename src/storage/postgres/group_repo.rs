use crate::storage::postgres::base::{
    delete_item, get_item, insert_item, update_item, SqlSerializer,
};
use crate::usecases::base_entities::AccessModelError;
use crate::usecases::group::entities::{
    Group, GroupForCreation, GroupsMemberBinding, GroupsPermissionBinding,
};
use crate::usecases::group::group_creator::CreateGroup;
use crate::usecases::group::group_disabler::DisableGroup;
use crate::usecases::group::group_get_item::GetGroup;
use crate::usecases::group::group_members_binder::GroupBindMember;
use crate::usecases::group::group_permissions_binder::GroupBindPermission;

use async_trait::async_trait;
use chrono;
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;

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

impl SqlSerializer<Group> for Group {
    fn from_sql_result(row: &Row) -> Group {
        Group::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}
#[async_trait]
impl GetGroup for GroupRepo {
    async fn get_group_by_id(&self, group_id: i32) -> Result<Group, AccessModelError> {
        get_item(&self.db_pool, GET_GROUP_BY_ID_QUERY, &[&group_id]).await
    }
}

#[async_trait]
impl CreateGroup for GroupRepo {
    async fn save_group_in_storage(
        &self,
        group_data: GroupForCreation,
    ) -> Result<Group, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&group_data.group_name, &now, &now, &false];
        insert_item(&self.db_pool, INSERT_GROUP_QUERY, params).await
    }
}

#[async_trait]
impl DisableGroup for GroupRepo {
    async fn disable_group_by_id(&self, group_id: i32) -> Result<(), AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &group_id];
        delete_item(&self.db_pool, DISABLE_GROUP_BY_ID_QUERY, params).await
    }
}

const GET_GROUPS_PERMISSION_BINDING_BY_PK_QUERY: &str =
    "SELECT permission_id, group_id, created_at, updated_at, is_deleted 
    FROM group_permissions 
    WHERE permission_id=$1 AND group_id=$2";
const ENABLE_GROUPS_PERMISSION_BINDING_QUERY: &str = "UPDATE group_permissions 
    SET is_deleted=FALSE, updated_at=$1 
    WHERE permission_id=$2 AND group_id=$3
    RETURNING permission_id, group_id, created_at, updated_at, is_deleted";
const ADD_PERMISSION_TO_GROUP_QUERY: &str = "INSERT INTO group_permissions 
    (permission_id, group_id, created_at, updated_at, is_deleted)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING permission_id, group_id, created_at, updated_at, is_deleted";
const DISABLE_GROUPS_PERMISSION_BINDING_QUERY: &str = "UPDATE group_permissions 
    SET is_deleted=TRUE, updated_at=$1 
    WHERE permission_id=$2 AND group_id=$3
    RETURNING permission_id, group_id, created_at, updated_at, is_deleted";

impl SqlSerializer<GroupsPermissionBinding> for GroupsPermissionBinding {
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
        get_item(
            &self.db_pool,
            GET_GROUPS_PERMISSION_BINDING_BY_PK_QUERY,
            &[&perm_id, &group_id],
        )
        .await
    }
    async fn enable_existed_groups_permission_binding(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &perm_id, &group_id];
        update_item(
            &self.db_pool,
            ENABLE_GROUPS_PERMISSION_BINDING_QUERY,
            params,
        )
        .await
    }
    async fn add_permission_to_group(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&perm_id, &group_id, &now, &now, &false];
        insert_item(&self.db_pool, ADD_PERMISSION_TO_GROUP_QUERY, params).await
    }
    async fn disable_existed_groups_permission_binding(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &perm_id, &group_id];
        update_item(
            &self.db_pool,
            DISABLE_GROUPS_PERMISSION_BINDING_QUERY,
            params,
        )
        .await
    }
}

const GET_GROUPS_MEMBER_BINDING_BY_PK_QUERY: &str =
    "SELECT user_id, group_id, created_at, updated_at, is_deleted 
    FROM group_members 
    WHERE user_id=$1 AND group_id=$2";
const ENABLE_GROUPS_MEMBER_BINDING_QUERY: &str = "UPDATE group_members 
    SET is_deleted=FALSE, updated_at=$1 
    WHERE user_id=$2 AND group_id=$3
    RETURNING user_id, group_id, created_at, updated_at, is_deleted";
const ADD_MEMBER_TO_GROUP_QUERY: &str = "INSERT INTO group_members 
    (user_id, group_id, created_at, updated_at, is_deleted)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING user_id, group_id, created_at, updated_at, is_deleted";
const DISABLE_GROUPS_MEMBER_BINDING_QUERY: &str = "UPDATE group_members 
    SET is_deleted=TRUE, updated_at=$1 
    WHERE user_id=$2 AND group_id=$3
    RETURNING user_id, group_id, created_at, updated_at, is_deleted";

impl SqlSerializer<GroupsMemberBinding> for GroupsMemberBinding {
    fn from_sql_result(row: &Row) -> GroupsMemberBinding {
        GroupsMemberBinding::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}

#[async_trait]
impl GroupBindMember for GroupRepo {
    async fn get_groups_member_binding(
        &self,
        group_id: i32,
        user_id: i32,
    ) -> Result<GroupsMemberBinding, AccessModelError> {
        get_item(
            &self.db_pool,
            GET_GROUPS_MEMBER_BINDING_BY_PK_QUERY,
            &[&user_id, &group_id],
        )
        .await
    }
    async fn enable_existed_groups_member_binding(
        &self,
        group_id: i32,
        user_id: i32,
    ) -> Result<GroupsMemberBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &user_id, &group_id];
        update_item(&self.db_pool, ENABLE_GROUPS_MEMBER_BINDING_QUERY, params).await
    }
    async fn add_member_to_group(
        &self,
        group_id: i32,
        user_id: i32,
    ) -> Result<GroupsMemberBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&user_id, &group_id, &now, &now, &false];
        insert_item(&self.db_pool, ADD_MEMBER_TO_GROUP_QUERY, params).await
    }
    async fn disable_existed_groups_member_binding(
        &self,
        group_id: i32,
        user_id: i32,
    ) -> Result<GroupsMemberBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &user_id, &group_id];
        update_item(&self.db_pool, DISABLE_GROUPS_MEMBER_BINDING_QUERY, params).await
    }
}
