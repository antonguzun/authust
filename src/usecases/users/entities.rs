use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
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
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> User {
        User {
            user_id,
            username,
            enabled,
            created_at: created_at.to_rfc3339(),
            updated_at: updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SingnedInfo {
    pub user_id: i32,
    pub jwt_token: String,
}

impl SingnedInfo {
    pub fn new(user_id: i32, jwt_token: String) -> SingnedInfo {
        SingnedInfo { user_id, jwt_token }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub expired_at: String,
    pub permissions: Vec<String>,
}

impl Claims {
    pub fn new(user_id: i32, expired_in_days: u32, permissions: Vec<String>) -> Claims {
        let expired_at =
            (chrono::Utc::now() + chrono::Duration::days(expired_in_days.into())).to_rfc3339();
        Claims {
            user_id,
            expired_at,
            permissions,
        }
    }
}
