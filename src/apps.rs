use crate::handlers::{
    create_user_handler, delete_user_by_id, get_user_by_id, sign_in_user_handler,
};
use actix_web::web;

pub fn init_api_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_by_id)
        .service(delete_user_by_id)
        .service(create_user_handler)
        .service(sign_in_user_handler);
}
