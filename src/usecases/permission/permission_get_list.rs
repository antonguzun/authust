use crate::usecases::base_entities::AccessModelError;
use crate::usecases::permission::entities::PermissionsFilters;
use crate::usecases::permission::errors::PermissionUCError;

use async_trait::async_trait;

use super::entities::PermissionsList;

#[async_trait]
pub trait GetPermissionsList {
    async fn get_permissions_by_filters(
        &self,
        filters: PermissionsFilters,
    ) -> Result<PermissionsList, AccessModelError>;
}

pub async fn get_permissions_by_filters(
    permission_access_model: &impl GetPermissionsList,
    filters: PermissionsFilters,
) -> Result<PermissionsList, PermissionUCError> {
    match permission_access_model
        .get_permissions_by_filters(filters)
        .await
    {
        Ok(permission) => Ok(permission),
        Err(AccessModelError::TemporaryError) => Err(PermissionUCError::TemporaryError),
        Err(_) => Err(PermissionUCError::FatalError),
    }
}
