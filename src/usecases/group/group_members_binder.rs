use crate::usecases::base_entities::AccessModelError;
use crate::usecases::group::entities::GroupsMemberBinding;
use crate::usecases::group::errors::GroupUCError;

use async_trait::async_trait;

#[async_trait]
pub trait GroupBindMember {
    async fn get_groups_member_binding(
        &self,
        group_id: i32,
        user_id: i32,
    ) -> Result<GroupsMemberBinding, AccessModelError>;
    async fn enable_existed_groups_member_binding(
        &self,
        group_id: i32,
        user_id: i32,
    ) -> Result<GroupsMemberBinding, AccessModelError>;
    async fn add_member_to_group(
        &self,
        group_id: i32,
        user_id: i32,
    ) -> Result<GroupsMemberBinding, AccessModelError>;
    async fn disable_existed_groups_member_binding(
        &self,
        group_id: i32,
        user_id: i32,
    ) -> Result<GroupsMemberBinding, AccessModelError>;
}

pub async fn bind_member_to_group(
    group_access_model: &impl GroupBindMember,
    group_id: i32,
    user_id: i32,
) -> Result<GroupsMemberBinding, GroupUCError> {
    match group_access_model
        .get_groups_member_binding(group_id, user_id)
        .await
    {
        Ok(binding) if binding.is_deleted == false => Ok(binding),
        Ok(binding) => match group_access_model
            .enable_existed_groups_member_binding(binding.group_id, binding.user_id)
            .await
        {
            Ok(binding) => Ok(binding),
            Err(AccessModelError::TemporaryError) => Err(GroupUCError::TemporaryError),
            Err(_) => Err(GroupUCError::FatalError),
        },
        Err(AccessModelError::NotFoundError) => match group_access_model
            .add_member_to_group(group_id, user_id)
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

pub async fn unbind_member_to_group(
    group_access_model: &impl GroupBindMember,
    group_id: i32,
    user_id: i32,
) -> Result<GroupsMemberBinding, GroupUCError> {
    match group_access_model
        .disable_existed_groups_member_binding(group_id, user_id)
        .await
    {
        Ok(binding) => Ok(binding),
        Err(AccessModelError::NotFoundError) => Err(GroupUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(GroupUCError::TemporaryError),
        Err(_) => Err(GroupUCError::FatalError),
    }
}
