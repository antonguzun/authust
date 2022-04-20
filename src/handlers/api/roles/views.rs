use crate::usecases::roles::entities::{Role, RoleMemberBinding, RolePermissionBinding};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoleView {
    pub role_id: i32,
    pub role_name: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_deleted: bool,
}

impl RoleView {
    pub fn new(role: Role) -> RoleView {
        RoleView {
            role_id: role.role_id,
            role_name: role.role_name,
            created_at: role.created_at.to_rfc3339(),
            updated_at: role.updated_at.to_rfc3339(),
            is_deleted: role.is_deleted,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BindingPermissionCreationScheme {
    pub permission_id: i32,
    pub role_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RolePermissionBindingView {
    pub permission_id: i32,
    pub role_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub is_deleted: bool,
}

impl RolePermissionBindingView {
    pub fn new(binding: RolePermissionBinding) -> RolePermissionBindingView {
        RolePermissionBindingView {
            permission_id: binding.permission_id,
            role_id: binding.role_id,
            created_at: binding.created_at.to_rfc3339(),
            updated_at: binding.updated_at.to_rfc3339(),
            is_deleted: binding.is_deleted,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BindingMemberCreationScheme {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RoleMemberBindingView {
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub is_deleted: bool,
}

impl RoleMemberBindingView {
    pub fn new(binding: RoleMemberBinding) -> RoleMemberBindingView {
        RoleMemberBindingView {
            user_id: binding.user_id,
            role_id: binding.role_id,
            created_at: binding.created_at.to_rfc3339(),
            updated_at: binding.updated_at.to_rfc3339(),
            is_deleted: binding.is_deleted,
        }
    }
}

#[derive(Deserialize)]
pub struct PermissionBindingQuery {
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Deserialize)]
pub struct MemberBindingQuery {
    pub role_id: i32,
    pub user_id: i32,
}
