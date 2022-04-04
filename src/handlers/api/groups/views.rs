use crate::usecases::group::entities::Group;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GroupView {
    pub group_id: i32,
    pub group_name: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_deleted: bool,
}

impl GroupView {
    pub fn new(group: Group) -> GroupView {
        GroupView {
            group_id: group.group_id,
            group_name: group.group_name,
            created_at: group.created_at.to_rfc3339(),
            updated_at: group.updated_at.to_rfc3339(),
            is_deleted: group.is_deleted,
        }
    }
}
