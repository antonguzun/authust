use crate::usecases::permission::entities::{Permission, PermissionForCreation};
use crate::usecases::user::errors::{AccessModelError, UserUCError};

use async_trait::async_trait;

#[async_trait]
pub trait CreatePermission {
    async fn save_permission_in_storage(
        &self,
        perm_data: PermissionForCreation,
    ) -> Result<Permission, AccessModelError>;
}

pub async fn create_new_permission(
    permission_access_model: &impl CreatePermission,
    perm_data: PermissionForCreation,
) -> Result<Permission, UserUCError> {
    match permission_access_model
        .save_permission_in_storage(perm_data)
        .await
    {
        Ok(permission) => Ok(permission),
        Err(AccessModelError::TemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}
