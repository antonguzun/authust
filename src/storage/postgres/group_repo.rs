use crate::usecases::group::entities::{Group, GroupForCreation};
use crate::usecases::group::errors::AccessModelError;
use crate::usecases::group::group_creator::CreateGroup;
use crate::usecases::group::group_disabler::DisableGroup;
use crate::usecases::group::group_get_item::GetGroup;
use async_trait::async_trait;
use chrono;
use deadpool_postgres::{Client, Pool};
use log::error;
use tokio_postgres::{Row, Statement};

pub struct GroupRepo {
    db_pool: Pool,
}

impl GroupRepo {
    pub fn new(db_pool: Pool) -> GroupRepo {
        GroupRepo { db_pool }
    }
}

const GET_BY_ID_QUERY: &str = "SELECT group_id, group_name, created_at, updated_at, is_deleted 
    FROM groups 
    WHERE group_id=$1";
const INSERT_GROUP_QUERY: &str =
    "INSERT INTO groups (group_name, created_at, updated_at, is_deleted) 
    VALUES ($1, $2, $3, $4) 
    RETURNING group_id, group_name, created_at, updated_at, is_deleted";
const DISABLE_GROUP_BY_ID_QUERY: &str = "UPDATE groups 
    SET is_deleted=TRUE, updated_at=$1 
    WHERE group_id=$2 AND is_deleted=FALSE";

async fn get_client(db_pool: &Pool) -> Result<Client, AccessModelError> {
    match db_pool.get().await {
        Ok(client) => Ok(client),
        Err(e) => {
            error!("{}", e);
            Err(AccessModelError::TemporaryError)
        }
    }
}

async fn prepare_stmt(client: &Client, query: &str) -> Result<Statement, AccessModelError> {
    match client.prepare(query).await {
        Ok(stmt) => Ok(stmt),
        Err(e) => {
            error!("{}", e);
            Err(AccessModelError::FatalError)
        }
    }
}

impl Group {
    fn from_sql_result(row: &Row) -> Group {
        Group::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
    }
}
#[async_trait]
impl GetGroup for GroupRepo {
    async fn get_group_by_id(&self, group_id: i32) -> Result<Group, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, GET_BY_ID_QUERY).await?;
        match client.query(&stmt, &[&group_id]).await {
            Ok(rows) if rows.len() == 1 => Ok(Group::from_sql_result(&rows[0])),
            Ok(_) => {
                error!("During getting group count of retirning rows not equals one");
                Err(AccessModelError::FatalError)
            }
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}
#[async_trait]
impl CreateGroup for GroupRepo {
    async fn save_group_in_storage(
        &self,
        group_data: GroupForCreation,
    ) -> Result<Group, AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, INSERT_GROUP_QUERY).await?;
        let now = chrono::Utc::now();
        match client
            .query(&stmt, &[&group_data.group_name, &now, &now, &false])
            .await
        {
            Ok(rows) if rows.len() == 1 => Ok(Group::from_sql_result(&rows[0])),
            Ok(_) => {
                error!("During creation group got count of retirning rows not equals one");
                Err(AccessModelError::FatalError)
            }
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}

#[async_trait]
impl DisableGroup for GroupRepo {
    async fn disable_group_by_id(&self, group_id: i32) -> Result<(), AccessModelError> {
        let client = get_client(&self.db_pool).await?;
        let stmt = prepare_stmt(&client, DISABLE_GROUP_BY_ID_QUERY).await?;

        let now = chrono::Utc::now();
        match client.execute(&stmt, &[&now, &group_id]).await {
            Ok(res) if res != 0 => Ok(()),
            Ok(_) => Err(AccessModelError::NotFoundError),
            Err(e) => {
                error!("{}", e);
                Err(AccessModelError::FatalError)
            }
        }
    }
}
