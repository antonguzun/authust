
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub group_id: i32,
    pub group_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl Group {
    pub fn new(
        group_id: i32,
        group_name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        is_deleted: bool,
    ) -> Group {
        Group {
            group_id,
            group_name,
            created_at,
            updated_at,
            is_deleted,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GroupForCreation {
    pub group_name: String,
}
