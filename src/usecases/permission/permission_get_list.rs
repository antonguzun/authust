use crate::usecases::base_entities::AccessModelError;
use crate::usecases::permission::entities::{Permission, PermissionsFilters};
use crate::usecases::permission::errors::PermissionUCError;

use async_trait::async_trait;

#[async_trait]
pub trait GetPermissionsList {
    async fn get_permissions_by_filters(
        &self,
        filters: PermissionsFilters,
    ) -> Result<Vec<Permission>, AccessModelError>;
}

pub async fn get_permissions_by_filters(
    permission_access_model: &impl GetPermissionsList,
    filters: PermissionsFilters,
) -> Result<Vec<Permission>, PermissionUCError> {
    match permission_access_model
        .get_permissions_by_filters(filters)
        .await
    {
        Ok(permission) => Ok(permission),
        Err(AccessModelError::NotFoundError) => Ok(vec![]),
        Err(AccessModelError::TemporaryError) => Err(PermissionUCError::TemporaryError),
        Err(_) => Err(PermissionUCError::FatalError),
    }
}
