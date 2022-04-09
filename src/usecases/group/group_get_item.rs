use crate::usecases::base_entities::AccessModelError;
use crate::usecases::group::entities::Group;
use crate::usecases::group::errors::GroupUCError;

use async_trait::async_trait;

#[async_trait]
pub trait GetGroup {
    async fn get_group_by_id(&self, group_id: i32) -> Result<Group, AccessModelError>;
}

pub async fn get_group_by_id(
    group_access_model: &impl GetGroup,
    group_id: i32,
) -> Result<Group, GroupUCError> {
    match group_access_model.get_group_by_id(group_id).await {
        Ok(group) => Ok(group),
        Err(AccessModelError::NotFoundError) => Err(GroupUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(GroupUCError::TemporaryError),
        Err(_) => Err(GroupUCError::FatalError),
    }
}
