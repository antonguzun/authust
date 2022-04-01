use crate::usecases::permission::entities::Permission;
use crate::usecases::user::errors::{AccessModelError, UserUCError};

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
) -> Result<Permission, UserUCError> {
    match permission_access_model
        .get_permission_by_id(permission_id)
        .await
    {
        Ok(permission) => Ok(permission),
        Err(AccessModelError::NotFoundError) => Err(UserUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(UserUCError::TemporaryError),
        Err(_) => Err(UserUCError::FatalError),
    }
}
