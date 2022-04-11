use crate::usecases::base_entities::AccessModelError;
use crate::usecases::group::entities::GroupsPermissionBinding;
use crate::usecases::group::errors::GroupUCError;

use async_trait::async_trait;

#[async_trait]
pub trait GroupBindPermission {
    async fn get_groups_permission_binding(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError>;
    async fn enable_existed_groups_permission_binding(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError>;
    async fn add_permission_to_group(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError>;
    async fn disable_existed_groups_permission_binding(
        &self,
        group_id: i32,
        perm_id: i32,
    ) -> Result<GroupsPermissionBinding, AccessModelError>;
}

pub async fn bind_permission_to_group(
    group_access_model: &impl GroupBindPermission,
    group_id: i32,
    perm_id: i32,
) -> Result<GroupsPermissionBinding, GroupUCError> {
    match group_access_model
        .get_groups_permission_binding(group_id, perm_id)
        .await
    {
        Ok(binding) if binding.is_deleted == false => Ok(binding),
        Ok(binding) => match group_access_model
            .enable_existed_groups_permission_binding(binding.group_id, binding.permission_id)
            .await
        {
            Ok(binding) => Ok(binding),
            Err(AccessModelError::TemporaryError) => Err(GroupUCError::TemporaryError),
            Err(_) => Err(GroupUCError::FatalError),
        },
        Err(AccessModelError::NotFoundError) => match group_access_model
            .add_permission_to_group(group_id, perm_id)
            .await
        {
            Ok(binding) => Ok(binding),
            Err(AccessModelError::TemporaryError) => Err(GroupUCError::TemporaryError),
            Err(_) => Err(GroupUCError::FatalError),
        },
        Err(AccessModelError::TemporaryError) => Err(GroupUCError::TemporaryError),
        Err(_) => Err(GroupUCError::FatalError),
    }
}

pub async fn unbind_permission_to_group(
    group_access_model: &impl GroupBindPermission,
    group_id: i32,
    perm_id: i32,
) -> Result<GroupsPermissionBinding, GroupUCError> {
    match group_access_model
        .disable_existed_groups_permission_binding(group_id, perm_id)
        .await
    {
        Ok(binding) => Ok(binding),
        Err(AccessModelError::NotFoundError) => Err(GroupUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(GroupUCError::TemporaryError),
        Err(_) => Err(GroupUCError::FatalError),
    }
}
