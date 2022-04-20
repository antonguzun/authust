use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Role {
    pub role_id: i32,
    pub role_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl Role {
    pub fn new(
        role_id: i32,
        role_name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        is_deleted: bool,
    ) -> Role {
        Role {
            role_id,
            role_name,
            created_at,
            updated_at,
            is_deleted,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RoleForCreation {
    pub role_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RolePermissionBinding {
    pub permission_id: i32,
    pub role_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl RolePermissionBinding {
    pub fn new(
        permission_id: i32,
        role_id: i32,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        is_deleted: bool,
    ) -> RolePermissionBinding {
        RolePermissionBinding {
            permission_id,
            role_id,
            created_at,
            updated_at,
            is_deleted,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RoleMemberBinding {
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl RoleMemberBinding {
    pub fn new(
        user_id: i32,
        role_id: i32,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        is_deleted: bool,
    ) -> RoleMemberBinding {
        RoleMemberBinding {
            user_id,
            role_id,
            created_at,
            updated_at,
            is_deleted,
        }
    }
}
