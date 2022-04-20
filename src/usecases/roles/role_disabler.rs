use crate::usecases::base_entities::AccessModelError;
use crate::usecases::roles::errors::RoleUCError;

use async_trait::async_trait;

#[async_trait]
pub trait DisableRole {
    async fn disable_role_by_id(&self, role_id: i32) -> Result<(), AccessModelError>;
}

pub async fn disable_role_by_id(
    role_access_model: &impl DisableRole,
    role_id: i32,
) -> Result<(), RoleUCError> {
    match role_access_model.disable_role_by_id(role_id).await {
        Ok(_) => Ok(()),
        Err(AccessModelError::NotFoundError) => Err(RoleUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
        Err(_) => Err(RoleUCError::FatalError),
    }
}
