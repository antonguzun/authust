use crate::storage::postgres::base::get_client;
use crate::usecases::base_entities::AccessModelError;
use deadpool_postgres::Pool;
use log::error;

pub async fn check_db(db_pool: &Pool) -> Result<(), AccessModelError> {
    let client = get_client(db_pool).await?;
    match client.simple_query("SELECT 1").await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{}", e);
            Err(AccessModelError::FatalError)
        }
    }
}
