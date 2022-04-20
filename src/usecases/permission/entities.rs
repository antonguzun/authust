use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Serialize, Deserialize)]
pub struct Permission {
    pub permission_id: i32,
    pub permission_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl Permission {
    pub fn new(
        permission_id: i32,
        permission_name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        is_deleted: bool,
    ) -> Permission {
        Permission {
            permission_id,
            permission_name,
            created_at,
            updated_at,
            is_deleted,
        }
    }
}

#[derive(Deserialize)]
pub struct PermissionForCreation {
    pub permission_name: String,
}

#[derive(Deserialize, Clone)]
pub struct PermissionsFilters {
    pub permission_id: Option<i32>,
    pub role_id: Option<i32>,
    pub is_deleted: Option<bool>,
    pub permission_name: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
