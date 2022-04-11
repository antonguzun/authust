use crate::usecases::group::entities::{Group, GroupsMemberBinding, GroupsPermissionBinding};
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

#[derive(Serialize, Deserialize)]
pub struct BindingPermissionCreationScheme {
    pub permission_id: i32,
    pub group_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GroupsPermissionBindingView {
    pub permission_id: i32,
    pub group_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub is_deleted: bool,
}

impl GroupsPermissionBindingView {
    pub fn new(binding: GroupsPermissionBinding) -> GroupsPermissionBindingView {
        GroupsPermissionBindingView {
            permission_id: binding.permission_id,
            group_id: binding.group_id,
            created_at: binding.created_at.to_rfc3339(),
            updated_at: binding.updated_at.to_rfc3339(),
            is_deleted: binding.is_deleted,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BindingMemberCreationScheme {
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GroupsMemberBindingView {
    pub user_id: i32,
    pub group_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub is_deleted: bool,
}

impl GroupsMemberBindingView {
    pub fn new(binding: GroupsMemberBinding) -> GroupsMemberBindingView {
        GroupsMemberBindingView {
            user_id: binding.user_id,
            group_id: binding.group_id,
            created_at: binding.created_at.to_rfc3339(),
            updated_at: binding.updated_at.to_rfc3339(),
            is_deleted: binding.is_deleted,
        }
    }
}

#[derive(Deserialize)]
pub struct PermissionBindingQuery {
    pub group_id: i32,
    pub permission_id: i32,
}

#[derive(Deserialize)]
pub struct MemberBindingQuery {
    pub group_id: i32,
    pub user_id: i32,
}
