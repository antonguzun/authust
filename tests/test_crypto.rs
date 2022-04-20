use argon2;
use authust::usecases::users::crypto;
mod constants;

#[test]
fn test_generate_hash() {
    let hash = crypto::generate_hash(&constants::TEST_PASSWORD).unwrap();
    assert_eq!(hash.to_string(), constants::TEST_PASSWORD_HASH.to_string());
    let matches = argon2::verify_encoded(&hash, constants::TEST_PASSWORD.as_bytes()).unwrap();
    assert!(matches);
}
