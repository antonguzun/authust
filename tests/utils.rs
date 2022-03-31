use actix_http::Request;
use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, web, App};
use rust_crud::apps::init_api_v1;
use rust_crud::common::{Config, Resources};
use std::fs;

#[path = "./constants.rs"]
//надо разобраться почему он сам не подхватывает модуль, хотя в соседнем файле таких проблем нет
mod constants;

async fn refresh_db(resources: &Resources) -> () {
    let client = resources.db_pool.get().await.unwrap();

    let migration_paths = fs::read_dir("./tests/migrations").unwrap();

    for path in migration_paths {
        let filename = path.unwrap().path().display().to_string();
        let query = &fs::read_to_string(&filename).unwrap();
        client.query(query, &[]).await.unwrap();
    }

    client.query("TRUNCATE TABLE users;", &[]).await.unwrap();
    client
        .query(
            "INSERT INTO users 
        (user_id, username, password_hash, enabled, created_at, updated_at, is_deleted)
        VALUES 
        (1, 'Ivan', '1234', TRUE, '2016-06-22 22:10:25+03', '2016-06-22 22:10:25+03', FALSE), 
        (2, 'Anton', $1, TRUE, '2022-06-22 22:10:25+00', '2022-06-22 22:10:25+00', FALSE), 
        (3, 'Godzilla', '1234', TRUE, '2022-06-22 22:10:25+00', '2022-06-22 22:10:25+00', FALSE)
        ON CONFLICT DO NOTHING;",
            &[&constants::TEST_PASSWORD_HASH],
        )
        .await
        .unwrap();
}

pub async fn init_test_service(
) -> impl Service<Request, Response = ServiceResponse<BoxBody>, Error = actix_web::Error> {
    let config = Config::create_config();
    let resources = Resources::create_resources(&config).await;
    refresh_db(&resources).await;
    test::init_service(
        App::new()
            .app_data(web::Data::new(config.clone()))
            .data(resources.clone())
            .service(web::scope("/api/v1").configure(init_api_v1)),
    )
    .await
}
