use crate::usecases::permission::entities::{Permission, PermissionsFilters};
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
pub struct Pagination {
    pub offset: i64,
    pub limit: i64,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PermissionListingView {
    pub permissions: Vec<Permission>,
    pub pagination: Pagination,
}
impl PermissionListingView {
    pub fn new(
        permissions: Vec<Permission>,
        offset: i64,
        limit: i64,
        total: i64,
    ) -> PermissionListingView {
        PermissionListingView {
            permissions,
            pagination: Pagination {
                offset,
                limit,
                total,
            },
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct PermissionsFiltersInputScheme {
    pub permission_id: Option<i32>,
    pub role_id: Option<i32>,
    pub is_deleted: Option<bool>,
    pub permission_name: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl PermissionsFiltersInputScheme {
    pub fn new_with_validation(
        data: PermissionsFiltersInputScheme,
    ) -> Result<PermissionsFilters, String> {
        let offset = match data.offset {
            None => 0,
            Some(offset) if 0 <= offset && offset < 9999 => offset,
            _ => return Err("wrong offset value".to_string()),
        };
        let limit = match data.limit {
            None => 100,
            Some(limit) if 0 < limit && limit <= 1000 => limit,
            _ => return Err("wrong limit value".to_string()),
        };
        Ok(PermissionsFilters {
            permission_id: data.permission_id,
            role_id: data.role_id,
            is_deleted: data.is_deleted,
            permission_name: data.permission_name,
            offset,
            limit,
        })
    }
}
