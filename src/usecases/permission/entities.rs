use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct PermissionForCreation {
    pub permission_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PermissionForDisabling {
    pub permission_id: i32,
}

impl PermissionForDisabling {
    pub fn new(permission_id: i32) -> PermissionForDisabling {
        PermissionForDisabling { permission_id }
    }
}
