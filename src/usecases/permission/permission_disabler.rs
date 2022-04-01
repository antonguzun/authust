use crate::usecases::permission::entities::PermissionForDisabling;
use crate::usecases::user::errors::{AccessModelError, UserUCError};

use async_trait::async_trait;

#[async_trait]
pub trait DisablePermission {
    async fn disable_permission_in_storage(
        &self,
        perm_data: PermissionForDisabling,
    ) -> Result<(), AccessModelError>;
}

pub async fn disable_permission_by_id(
    permission_access_model: &impl DisablePermission,
    perm_data: PermissionForDisabling,
) -> Result<(), UserUCError> {
    match permission_access_model
        .disable_permission_in_storage(perm_data)
        .await
    {
        Ok(_) => Ok(()),
        Err(AccessModelError::NotFoundError) => Err(UserUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}
