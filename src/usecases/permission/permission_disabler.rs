use crate::usecases::base_entities::AccessModelError;
use crate::usecases::permission::errors::PermissionUCError;

use async_trait::async_trait;

#[async_trait]
pub trait DisablePermission {
    async fn disable_permission_in_storage(
        &self,
        permission_id: i32,
    ) -> Result<(), AccessModelError>;
}

pub async fn disable_permission_by_id(
    permission_access_model: &impl DisablePermission,
    permission_id: i32,
) -> Result<(), PermissionUCError> {
    match permission_access_model
        .disable_permission_in_storage(permission_id)
        .await
    {
        Ok(_) => Ok(()),
        Err(AccessModelError::NotFoundError) => Err(PermissionUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(PermissionUCError::TemporaryError),
        Err(_) => Err(PermissionUCError::FatalError),
    }
}
