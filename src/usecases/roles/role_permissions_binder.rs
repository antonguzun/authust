use crate::usecases::base_entities::AccessModelError;
use crate::usecases::roles::entities::RolePermissionBinding;
use crate::usecases::roles::errors::RoleUCError;

use async_trait::async_trait;

#[async_trait]
pub trait RoleBindPermission {
    async fn get_role_permission_binding(
        &self,
        role_id: i32,
        perm_id: i32,
    ) -> Result<RolePermissionBinding, AccessModelError>;
    async fn enable_existed_role_permission_binding(
        &self,
        role_id: i32,
        perm_id: i32,
    ) -> Result<RolePermissionBinding, AccessModelError>;
    async fn add_permission_to_role(
        &self,
        role_id: i32,
        perm_id: i32,
    ) -> Result<RolePermissionBinding, AccessModelError>;
    async fn disable_existed_role_permission_binding(
        &self,
        role_id: i32,
        perm_id: i32,
    ) -> Result<RolePermissionBinding, AccessModelError>;
}

pub async fn bind_permission_to_role(
    role_access_model: &impl RoleBindPermission,
    role_id: i32,
    perm_id: i32,
) -> Result<RolePermissionBinding, RoleUCError> {
    match role_access_model
        .get_role_permission_binding(role_id, perm_id)
        .await
    {
        Ok(binding) if binding.is_deleted == false => Ok(binding),
        Ok(binding) => match role_access_model
            .enable_existed_role_permission_binding(binding.role_id, binding.permission_id)
            .await
        {
            Ok(binding) => Ok(binding),
            Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
            Err(_) => Err(RoleUCError::FatalError),
        },
        Err(AccessModelError::NotFoundError) => match role_access_model
            .add_permission_to_role(role_id, perm_id)
            .await
        {
            Ok(binding) => Ok(binding),
            Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
            Err(_) => Err(RoleUCError::FatalError),
        },
        Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
        Err(_) => Err(RoleUCError::FatalError),
    }
}

pub async fn unbind_permission_to_role(
    role_access_model: &impl RoleBindPermission,
    role_id: i32,
    perm_id: i32,
) -> Result<RolePermissionBinding, RoleUCError> {
    match role_access_model
        .disable_existed_role_permission_binding(role_id, perm_id)
        .await
    {
        Ok(binding) => Ok(binding),
        Err(AccessModelError::NotFoundError) => Err(RoleUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
        Err(_) => Err(RoleUCError::FatalError),
    }
}
