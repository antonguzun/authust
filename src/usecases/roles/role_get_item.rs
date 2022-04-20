use crate::usecases::base_entities::AccessModelError;
use crate::usecases::roles::entities::Role;
use crate::usecases::roles::errors::RoleUCError;

use async_trait::async_trait;

#[async_trait]
pub trait GetRole {
    async fn get_role_by_id(&self, role_id: i32) -> Result<Role, AccessModelError>;
}

pub async fn get_role_by_id(
    role_access_model: &impl GetRole,
    role_id: i32,
) -> Result<Role, RoleUCError> {
    match role_access_model.get_role_by_id(role_id).await {
        Ok(role) => Ok(role),
        Err(AccessModelError::NotFoundError) => Err(RoleUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
        Err(_) => Err(RoleUCError::FatalError),
    }
}
