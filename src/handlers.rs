use crate::common::Resources;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use web::Data;

#[derive(Serialize, Deserialize)]
pub struct EntityCreate {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct EntityQuery {
    offset: Option<i64>,
    limit: Option<i64>,
}

#[post("/entity")]
pub async fn create_entity(
    entity: web::Json<EntityCreate>,
    resources: Data<Resources>,
) -> impl Responder {
    let client = resources.db_pool.get().await.unwrap();
    let stmt = client
        .prepare("INSERT INTO entities (name) VALUES ($1) RETURNING entity_id")
        .await
        .unwrap();
    let rows = client.query(&stmt, &[&entity.name]).await.unwrap();
    let entity_id: i32 = rows[0].get(0);
    HttpResponse::Created().json(Entity {
        id: entity_id,
        name: entity.name.clone(),
    })
}

#[delete("/entity/{entity_id}")]
pub async fn delete_entity(req: HttpRequest, resources: Data<Resources>) -> impl Responder {
    let entity_id: i32 = req.match_info().get("entity_id").unwrap().parse().unwrap();
    let client = resources.db_pool.get().await.unwrap();
    let stmt = client
        .prepare("DELETE FROM entities where entity_id=$1")
        .await
        .unwrap();
    client.execute(&stmt, &[&entity_id]).await;
    HttpResponse::NoContent()
}

#[get("/entity/{entity_id}")]
pub async fn get_entity(req: HttpRequest, resources: Data<Resources>) -> impl Responder {
    let entity_id: i32 = req.match_info().get("entity_id").unwrap().parse().unwrap();
    let client = resources.db_pool.get().await.unwrap();
    let stmt = client
        .prepare("SELECT entity_id, name FROM entities where entity_id=$1")
        .await
        .unwrap();
    let row = client.query_one(&stmt, &[&entity_id]).await;
    match row {
        Ok(data) => HttpResponse::Ok().json(Entity {
            id: data.get(0),
            name: data.get(1),
        }),
        Err(_) => HttpResponse::NotFound().body("Not Found"),
    }
}

#[get("/entity")]
pub async fn get_entities(
    q: web::Query<EntityQuery>,
    resources: Data<Resources>,
) -> impl Responder {
    let offset = q.offset.unwrap_or(0);
    let limit = q.limit.unwrap_or(100);
    let client = resources.db_pool.get().await.unwrap();
    let stmt = client
        .prepare("SELECT entity_id, name FROM entities LIMIT $1 OFFSET $2")
        .await
        .unwrap();
    let rows = client.query(&stmt, &[&limit, &offset]).await.unwrap();
    let entities: Vec<Entity> = rows
        .iter()
        .map(|row| Entity {
            id: row.get(0),
            name: row.get(1),
        })
        .collect();
    HttpResponse::Ok().json(entities)
}
