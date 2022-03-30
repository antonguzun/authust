use argon2;
use rust_crud::usecases::user::crypto;

#[test]
fn test_generate_hash() {
    let password = "hello";
    let hash = crypto::generate_hash(&password).unwrap();
    assert_eq!(hash.to_string(), "$argon2i$v=19$m=4096,t=3,p=1$MjJmNjVlNzktNDk2YS00YjQ4LThhYmMtZjgzZTFlNTJhYTRl$GrBGOuJ9PznSgBOp0e5sdkMf2KAfgnubSh37Oq0HAzw".to_string());
    let matches = argon2::verify_encoded(&hash, password.as_bytes()).unwrap();
    assert!(matches);
}
