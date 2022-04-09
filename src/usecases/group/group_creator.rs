use crate::usecases::base_entities::AccessModelError;
use crate::usecases::group::entities::{Group, GroupForCreation};
use crate::usecases::group::errors::GroupUCError;

use async_trait::async_trait;

#[async_trait]
pub trait CreateGroup {
    async fn save_group_in_storage(
        &self,
        group_data: GroupForCreation,
    ) -> Result<Group, AccessModelError>;
}

pub async fn create_new_group(
    group_access_model: &impl CreateGroup,
    group_data: GroupForCreation,
) -> Result<Group, GroupUCError> {
    match group_access_model.save_group_in_storage(group_data).await {
        Ok(group) => Ok(group),
        Err(AccessModelError::AlreadyExists) => Err(GroupUCError::AlreadyExists),
        Err(AccessModelError::TemporaryError) => Err(GroupUCError::TemporaryError),
        Err(_) => Err(GroupUCError::FatalError),
    }
}
