use crate::common::Resources;
use crate::storage;
use crate::usecases::user::entities::UserContent;
use crate::usecases::user::get_user;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::error;
use web::Data;

// #[derive(Serialize, Deserialize)]
// pub struct EntityQuery {
//     offset: Option<i64>,
//     limit: Option<i64>,
// }
//
// #[get("/entity")]
// pub async fn get_entities(
//     q: web::Query<EntityQuery>,
//     resources: Data<Resources>,
// ) -> impl Responder {
//     let offset = q.offset.unwrap_or(0);
//     let limit = q.limit.unwrap_or(100);
//     let client = resources.db_pool.get().await.unwrap();
//     let stmt = client
//         .prepare("SELECT entity_id, name FROM entities LIMIT $1 OFFSET $2")
//         .await
//         .unwrap();
//     let rows = client.query(&stmt, &[&limit, &offset]).await.unwrap();
//     let entities: Vec<Entity> = rows
//         .iter()
//         .map(|row| Entity::new(row.get(0), row.get(1)))
//         .collect();
//     HttpResponse::Ok().json(entities)
// }

#[get("/user/postgres/{user_id}")]
pub async fn get_user_by_id(user_id: web::Path<u32>, resources: Data<Resources>) -> impl Responder {
    let user_id = user_id.into_inner() as i32;
    let user_repo = storage::postgres::user_repo::UserRepo::new(resources.db_pool.clone());
    match get_user::get_user_by_id(&user_repo, user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(get_user::UserUCError::NotFoundError) => HttpResponse::NotFound().body("Not Found"),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[delete("/user/postgres/{user_id}")]
pub async fn delete_user_by_id(
    user_id: web::Path<u32>,
    resources: Data<Resources>,
) -> impl Responder {
    let user_id = user_id.into_inner() as i32;
    let user_repo = storage::postgres::user_repo::UserRepo::new(resources.db_pool.clone());
    match get_user::remove_user_by_id(&user_repo, user_id).await {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(get_user::UserUCError::NotFoundError) => HttpResponse::NoContent().body(""),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}

#[post("/user/postgres")]
pub async fn create_user(
    user_data: web::Json<UserContent>,
    resources: Data<Resources>,
) -> impl Responder {
    let user_repo = storage::postgres::user_repo::UserRepo::new(resources.db_pool.clone());
    match get_user::create_user(&user_repo, user_data.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => {
            error!("usecase error");
            HttpResponse::InternalServerError().body("internal error")
        }
    }
}
