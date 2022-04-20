use crate::storage::postgres::base::{
    delete_item, get_item, insert_item, update_item, SqlSerializer,
};
use crate::usecases::base_entities::AccessModelError;
use crate::usecases::roles::entities::{
    Role, RoleForCreation, RoleMemberBinding, RolePermissionBinding,
};
use crate::usecases::roles::role_creator::CreateRole;
use crate::usecases::roles::role_disabler::DisableRole;
use crate::usecases::roles::role_get_item::GetRole;
use crate::usecases::roles::role_members_binder::RoleBindMember;
use crate::usecases::roles::role_permissions_binder::RoleBindPermission;

use async_trait::async_trait;
use chrono;
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;

pub struct RoleRepo {
    db_pool: Pool,
}

impl RoleRepo {
    pub fn new(db_pool: Pool) -> RoleRepo {
        RoleRepo { db_pool }
    }
}

const GET_ROLE_BY_ID_QUERY: &str = "SELECT role_id, role_name, created_at, updated_at, is_deleted 
    FROM roles 
    WHERE role_id=$1";
const INSERT_ROLE_QUERY: &str = "INSERT INTO roles (role_name, created_at, updated_at, is_deleted) 
    VALUES ($1, $2, $3, $4) 
    RETURNING role_id, role_name, created_at, updated_at, is_deleted";
const DISABLE_ROLE_BY_ID_QUERY: &str = "UPDATE roles 
    SET is_deleted=TRUE, updated_at=$1 
    WHERE role_id=$2 AND is_deleted=FALSE";

impl SqlSerializer<Role> for Role {
    fn from_sql_result(row: &Row) -> Role {
        Role::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}
#[async_trait]
impl GetRole for RoleRepo {
    async fn get_role_by_id(&self, role_id: i32) -> Result<Role, AccessModelError> {
        get_item(&self.db_pool, GET_ROLE_BY_ID_QUERY, &[&role_id]).await
    }
}

#[async_trait]
impl CreateRole for RoleRepo {
    async fn save_role_in_storage(
        &self,
        role_data: RoleForCreation,
    ) -> Result<Role, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&role_data.role_name, &now, &now, &false];
        insert_item(&self.db_pool, INSERT_ROLE_QUERY, params).await
    }
}

#[async_trait]
impl DisableRole for RoleRepo {
    async fn disable_role_by_id(&self, role_id: i32) -> Result<(), AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &role_id];
        delete_item(&self.db_pool, DISABLE_ROLE_BY_ID_QUERY, params).await
    }
}

const GET_ROLE_PERMISSION_BINDING_BY_PK_QUERY: &str =
    "SELECT permission_id, role_id, created_at, updated_at, is_deleted 
    FROM role_permissions 
    WHERE permission_id=$1 AND role_id=$2";
const ENABLE_ROLE_PERMISSION_BINDING_QUERY: &str = "UPDATE role_permissions 
    SET is_deleted=FALSE, updated_at=$1 
    WHERE permission_id=$2 AND role_id=$3
    RETURNING permission_id, role_id, created_at, updated_at, is_deleted";
const ADD_PERMISSION_TO_ROLE_QUERY: &str = "INSERT INTO role_permissions 
    (permission_id, role_id, created_at, updated_at, is_deleted)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING permission_id, role_id, created_at, updated_at, is_deleted";
const DISABLE_ROLE_PERMISSION_BINDING_QUERY: &str = "UPDATE role_permissions 
    SET is_deleted=TRUE, updated_at=$1 
    WHERE permission_id=$2 AND role_id=$3
    RETURNING permission_id, role_id, created_at, updated_at, is_deleted";

impl SqlSerializer<RolePermissionBinding> for RolePermissionBinding {
    fn from_sql_result(row: &Row) -> RolePermissionBinding {
        RolePermissionBinding::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}

#[async_trait]
impl RoleBindPermission for RoleRepo {
    async fn get_role_permission_binding(
        &self,
        role_id: i32,
        perm_id: i32,
    ) -> Result<RolePermissionBinding, AccessModelError> {
        get_item(
            &self.db_pool,
            GET_ROLE_PERMISSION_BINDING_BY_PK_QUERY,
            &[&perm_id, &role_id],
        )
        .await
    }
    async fn enable_existed_role_permission_binding(
        &self,
        role_id: i32,
        perm_id: i32,
    ) -> Result<RolePermissionBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &perm_id, &role_id];
        update_item(&self.db_pool, ENABLE_ROLE_PERMISSION_BINDING_QUERY, params).await
    }
    async fn add_permission_to_role(
        &self,
        role_id: i32,
        perm_id: i32,
    ) -> Result<RolePermissionBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&perm_id, &role_id, &now, &now, &false];
        insert_item(&self.db_pool, ADD_PERMISSION_TO_ROLE_QUERY, params).await
    }
    async fn disable_existed_role_permission_binding(
        &self,
        role_id: i32,
        perm_id: i32,
    ) -> Result<RolePermissionBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &perm_id, &role_id];
        update_item(&self.db_pool, DISABLE_ROLE_PERMISSION_BINDING_QUERY, params).await
    }
}

const GET_ROLE_MEMBER_BINDING_BY_PK_QUERY: &str =
    "SELECT user_id, role_id, created_at, updated_at, is_deleted 
    FROM role_members 
    WHERE user_id=$1 AND role_id=$2";
const ENABLE_ROLE_MEMBER_BINDING_QUERY: &str = "UPDATE role_members 
    SET is_deleted=FALSE, updated_at=$1 
    WHERE user_id=$2 AND role_id=$3
    RETURNING user_id, role_id, created_at, updated_at, is_deleted";
const ADD_MEMBER_TO_ROLE_QUERY: &str = "INSERT INTO role_members 
    (user_id, role_id, created_at, updated_at, is_deleted)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING user_id, role_id, created_at, updated_at, is_deleted";
const DISABLE_ROLE_MEMBER_BINDING_QUERY: &str = "UPDATE role_members 
    SET is_deleted=TRUE, updated_at=$1 
    WHERE user_id=$2 AND role_id=$3
    RETURNING user_id, role_id, created_at, updated_at, is_deleted";

impl SqlSerializer<RoleMemberBinding> for RoleMemberBinding {
    fn from_sql_result(row: &Row) -> RoleMemberBinding {
        RoleMemberBinding::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}

#[async_trait]
impl RoleBindMember for RoleRepo {
    async fn get_role_member_binding(
        &self,
        role_id: i32,
        user_id: i32,
    ) -> Result<RoleMemberBinding, AccessModelError> {
        get_item(
            &self.db_pool,
            GET_ROLE_MEMBER_BINDING_BY_PK_QUERY,
            &[&user_id, &role_id],
        )
        .await
    }
    async fn enable_existed_role_member_binding(
        &self,
        role_id: i32,
        user_id: i32,
    ) -> Result<RoleMemberBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &user_id, &role_id];
        update_item(&self.db_pool, ENABLE_ROLE_MEMBER_BINDING_QUERY, params).await
    }
    async fn add_member_to_role(
        &self,
        role_id: i32,
        user_id: i32,
    ) -> Result<RoleMemberBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&user_id, &role_id, &now, &now, &false];
        insert_item(&self.db_pool, ADD_MEMBER_TO_ROLE_QUERY, params).await
    }
    async fn disable_existed_role_member_binding(
        &self,
        role_id: i32,
        user_id: i32,
    ) -> Result<RoleMemberBinding, AccessModelError> {
        let now = chrono::Utc::now();
        let params: &[&(dyn ToSql + Sync)] = &[&now, &user_id, &role_id];
        update_item(&self.db_pool, DISABLE_ROLE_MEMBER_BINDING_QUERY, params).await
    }
}
