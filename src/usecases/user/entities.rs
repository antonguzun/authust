use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub enabled: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize)]
pub struct InputRawUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserForCreation {
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn new(
        user_id: i32,
        username: String,
        enabled: bool,
        created_at: SystemTime,
        updated_at: SystemTime,
    ) -> User {
        User {
            user_id,
            username,
            enabled,
            created_at,
            updated_at,
        }
    }
}
