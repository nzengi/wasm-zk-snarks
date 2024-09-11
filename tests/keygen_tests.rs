use wasm_zk_snarks::keygen::{generate_key_with_timestamp, KeyWithExpiration};  // Kütüphane modüllerini doğru import et
use wasm_zk_snarks::utils::current_timestamp;

#[test]
fn test_key_generation() {
    let (key, timestamp) = generate_key_with_timestamp();
    assert_eq!(key.len(), 32);
    assert!(timestamp > 0);
}

#[test]
fn test_key_expiration() {
    let key = KeyWithExpiration {
        key: [0u8; 32],
        created_at: current_timestamp(),
        expires_in: 60,
    };
    assert!(!key.is_expired());
}
