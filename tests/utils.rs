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

const USERS_FIXTURE: &str = "INSERT INTO users 
    (user_id, username, password_hash, enabled, created_at, updated_at, is_deleted)
    VALUES 
    (1, 'Ivan', '1234', TRUE, '2016-06-22 22:10:25+03', '2016-06-22 22:10:25+03', FALSE), 
    (2, 'Anton', '$argon2i$v=19$m=4096,t=3,p=1$MjJmNjVlNzktNDk2YS00YjQ4LThhYmMtZjgzZTFlNTJhYTRl$GrBGOuJ9PznSgBOp0e5sdkMf2KAfgnubSh37Oq0HAzw', TRUE, '2022-06-22 22:10:25+00', '2022-06-22 22:10:25+00', FALSE), 
    (3, 'Godzilla', '1234', TRUE, now(), now(), FALSE)
    ON CONFLICT DO NOTHING";
const PERMISSIONS_FIXTURE: &str =
    "INSERT INTO permissions (permission_id, permission_name, created_at, updated_at, is_deleted)
    VALUES 
    (1, 'PERM_1', '2016-06-22 22:10:25+03', '2016-06-22 22:10:25+03', FALSE), 
    (2, 'PERM_2', '2022-06-22 22:10:25+00', '2022-06-22 22:10:25+00', FALSE), 
    (3, 'PERM_3', now(), now(), TRUE)
    ON CONFLICT DO NOTHING";
const GROUPS_FUXTURE: &str =
    "INSERT INTO groups (group_id, group_name, created_at, updated_at, is_deleted)
    VALUES 
    (1, 'GROUP_1', '2016-06-22 22:10:25+03', '2016-06-22 22:10:25+03', FALSE), 
    (2, 'GROUP_2', '2022-06-22 22:10:25+00', '2022-06-22 22:10:25+00', FALSE), 
    (3, 'GROUP_3', now(), now(), TRUE)
    ON CONFLICT DO NOTHING";

async fn refresh_db(resources: &Resources) -> () {
    let client = resources.db_pool.get().await.unwrap();

    let migration_paths = fs::read_dir("./tests/migrations").unwrap();

    for path in migration_paths {
        let filename = path.unwrap().path().display().to_string();
        let query = &fs::read_to_string(&filename).unwrap();
        client.batch_execute(query).await.unwrap();
    }
    client
        .simple_query("TRUNCATE TABLE users, permissions, groups, group_permissions CASCADE")
        .await
        .unwrap();
    client.simple_query(USERS_FIXTURE).await.unwrap();

    client.simple_query(PERMISSIONS_FIXTURE).await.unwrap();
    client.simple_query(GROUPS_FUXTURE).await.unwrap();
}

pub enum TestTables {
    Users,
    Permissions,
    Groups,
}

pub async fn resources() -> Resources {
    let config = Config::create_config();
    Resources::create_resources(&config).await
}

pub async fn flash_table(table: TestTables) {
    let client = resources().await.db_pool.get().await.unwrap();
    match table {
        TestTables::Users => client
            .simple_query("TRUNCATE TABLE users CASCADE")
            .await
            .unwrap(),
        TestTables::Permissions => client
            .simple_query("TRUNCATE TABLE permissions CASCADE")
            .await
            .unwrap(),
        TestTables::Groups => client
            .simple_query("TRUNCATE TABLE groups CASCADE")
            .await
            .unwrap(),
    };
}

pub async fn write_default_fixture_for_table(table: TestTables) {
    let client = resources().await.db_pool.get().await.unwrap();
    match table {
        TestTables::Users => client.simple_query(USERS_FIXTURE).await.unwrap(),
        TestTables::Permissions => client.simple_query(PERMISSIONS_FIXTURE).await.unwrap(),
        TestTables::Groups => client.simple_query(GROUPS_FUXTURE).await.unwrap(),
    };
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
