use crate::usecases::base_entities::AccessModelError;
use crate::usecases::permission::entities::Permission;
use crate::usecases::permission::errors::PermissionUCError;

use async_trait::async_trait;

#[async_trait]
pub trait GetPermission {
    async fn get_permission_by_id(
        &self,
        permission_id: i32,
    ) -> Result<Permission, AccessModelError>;
}

pub async fn get_permission_by_id(
    permission_access_model: &impl GetPermission,
    permission_id: i32,
) -> Result<Permission, PermissionUCError> {
    match permission_access_model
        .get_permission_by_id(permission_id)
        .await
    {
        Ok(permission) => Ok(permission),
        Err(AccessModelError::NotFoundError) => Err(PermissionUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(PermissionUCError::TemporaryError),
        Err(_) => Err(PermissionUCError::FatalError),
    }
}
