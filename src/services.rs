use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use std::error;

#[derive(Serialize, Deserialize)]
pub struct Entity {
    id: i32,
    name: String,
}

impl Entity {
    pub fn new(id: i32, name: String) -> Entity {
        Entity { id, name }
    }
}

pub async fn get_entity_by_id(
    db_pool: Pool,
    entity_id: i32,
) -> Result<Option<Entity>, Box<dyn error::Error>> {
    let client = db_pool.get().await?;
    let stmt = client
        .prepare("SELECT entity_id, name FROM entities where entity_id=$1")
        .await?;
    let rows = client.query(&stmt, &[&entity_id]).await?;
    match rows.len() {
        0 => Ok(None),
        _ => Ok(Some(Entity::new(entity_id, rows[0].get(1)))),
    }
}
