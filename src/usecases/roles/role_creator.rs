use crate::usecases::base_entities::AccessModelError;
use crate::usecases::roles::entities::{Role, RoleForCreation};
use crate::usecases::roles::errors::RoleUCError;

use async_trait::async_trait;

#[async_trait]
pub trait CreateRole {
    async fn save_role_in_storage(
        &self,
        role_data: RoleForCreation,
    ) -> Result<Role, AccessModelError>;
}

pub async fn create_new_role(
    role_access_model: &impl CreateRole,
    role_data: RoleForCreation,
) -> Result<Role, RoleUCError> {
    match role_access_model.save_role_in_storage(role_data).await {
        Ok(role) => Ok(role),
        Err(AccessModelError::AlreadyExists) => Err(RoleUCError::AlreadyExists),
        Err(AccessModelError::TemporaryError) => Err(RoleUCError::TemporaryError),
        Err(_) => Err(RoleUCError::FatalError),
    }
}
