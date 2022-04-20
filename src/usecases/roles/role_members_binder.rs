use crate::usecases::base_entities::AccessModelError;
use crate::usecases::roles::entities::RoleMemberBinding;
use crate::usecases::roles::errors::RoleUCError;

use async_trait::async_trait;

#[async_trait]
pub trait RoleBindMember {
    async fn get_role_member_binding(
        &self,
        role_id: i32,
        user_id: i32,
    ) -> Result<RoleMemberBinding, AccessModelError>;
    async fn enable_existed_role_member_binding(
        &self,
        role_id: i32,
        user_id: i32,
    ) -> Result<RoleMemberBinding, AccessModelError>;
    async fn add_member_to_role(
        &self,
        role_id: i32,
        user_id: i32,
    ) -> Result<RoleMemberBinding, AccessModelError>;
    async fn disable_existed_role_member_binding(
        &self,
        role_id: i32,
        user_id: i32,
    ) -> Result<RoleMemberBinding, AccessModelError>;
}

pub async fn bind_member_to_role(
    role_access_model: &impl RoleBindMember,
    role_id: i32,
    user_id: i32,
) -> Result<RoleMemberBinding, RoleUCError> {
    match role_access_model
        .get_role_member_binding(role_id, user_id)
        .await
    {
        Ok(binding) if binding.is_deleted == false => Ok(binding),
        Ok(binding) => match role_access_model
            .enable_existed_role_member_binding(binding.role_id, binding.user_id)
            .await
        {
            Ok(binding) => Ok(binding),
            Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
            Err(_) => Err(RoleUCError::FatalError),
        },
        Err(AccessModelError::NotFoundError) => {
            match role_access_model.add_member_to_role(role_id, user_id).await {
                Ok(binding) => Ok(binding),
                Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
                Err(_) => Err(RoleUCError::FatalError),
            }
        }
        Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
        Err(_) => Err(RoleUCError::FatalError),
    }
}

pub async fn unbind_member_to_role(
    role_access_model: &impl RoleBindMember,
    role_id: i32,
    user_id: i32,
) -> Result<RoleMemberBinding, RoleUCError> {
    match role_access_model
        .disable_existed_role_member_binding(role_id, user_id)
        .await
    {
        Ok(binding) => Ok(binding),
        Err(AccessModelError::NotFoundError) => Err(RoleUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
        Err(_) => Err(RoleUCError::FatalError),
    }
}
