use crate::usecases::permission::entities::Permission;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PermissionView {
    pub permission_id: i32,
    pub permission_name: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_deleted: bool,
}

impl PermissionView {
    pub fn new(permission: Permission) -> PermissionView {
        PermissionView {
            permission_id: permission.permission_id,
            permission_name: permission.permission_name,
            created_at: permission.created_at.to_rfc3339(),
            updated_at: permission.updated_at.to_rfc3339(),
            is_deleted: permission.is_deleted,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PermissionListingView {
    pub permissions: Vec<Permission>,
}
impl PermissionListingView {
    pub fn new(permissions: Vec<Permission>) -> PermissionListingView {
        PermissionListingView { permissions }
    }
}
