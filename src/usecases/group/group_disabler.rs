use crate::usecases::base_entities::AccessModelError;
use crate::usecases::group::errors::GroupUCError;

use async_trait::async_trait;

#[async_trait]
pub trait DisableGroup {
    async fn disable_group_by_id(&self, group_iid: i32) -> Result<(), AccessModelError>;
}

pub async fn disable_group_by_id(
    group_access_model: &impl DisableGroup,
    group_id: i32,
) -> Result<(), GroupUCError> {
    match group_access_model.disable_group_by_id(group_id).await {
        Ok(_) => Ok(()),
        Err(AccessModelError::NotFoundError) => Err(GroupUCError::NotFoundError),
        Err(AccessModelError::TemporaryError) => Err(GroupUCError::TemporaryError),
        Err(_) => Err(GroupUCError::FatalError),
    }
}
