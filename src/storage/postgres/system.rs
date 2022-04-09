use crate::storage::postgres::base::{get_client, prepare_stmt};
use crate::usecases::base_entities::AccessModelError;
use deadpool_postgres::Pool;
use log::error;

const READY_QUERY: &str = "SELECT 1";

pub async fn check_db(db_pool: &Pool) -> Result<(), AccessModelError> {
    let client = get_client(db_pool).await?;
    let stmt = prepare_stmt(&client, READY_QUERY).await?;
    match client.query(&stmt, &[]).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{}", e);
            Err(AccessModelError::FatalError)
        }
    }
}
