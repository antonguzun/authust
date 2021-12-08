use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    user_id: i32,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserContent {
    pub name: String,
}

impl User {
    pub fn new(user_id: i32, name: String) -> User {
        User { user_id, name }
    }
}
