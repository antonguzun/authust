use crate::handlers::api::permissions::handlers::{
    create_permission_handler, disable_permission_handler,
};
use crate::handlers::api::users::{
    create_user_handler, delete_user_by_id, get_user_by_id, sign_in_user_handler,
};
use crate::handlers::system::handlers::{ping_handler, ready_handler};

use actix_web::web;

pub fn init_api_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_by_id)
        .service(delete_user_by_id)
        .service(create_user_handler)
        .service(sign_in_user_handler)
        .service(create_permission_handler)
        .service(disable_permission_handler);
}

pub fn init_system(cfg: &mut web::ServiceConfig) {
    cfg.service(ready_handler).service(ping_handler);
}
