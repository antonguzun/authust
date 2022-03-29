use log::error;

use argon2::{self, Config};
static SALT: &[u8] = "22f65e79-496a-4b48-8abc-f83e1e52aa4e".as_bytes();

pub fn generate_hash(password: &str) -> Result<String, &str> {
    let password = "hello".as_bytes();
    let config = Config::default();
    match argon2::hash_encoded(password, SALT, &config) {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => {
            error!("hashing password error: {}", e);
            Err("Some err")
        }
    }
}
