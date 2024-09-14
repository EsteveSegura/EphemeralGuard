extern crate ephemeral_guard;

use ephemeral_guard::db::models::secret_data::SecretData;
use ephemeral_guard::db::storage::principal_store::PrincipalStore;
use ephemeral_guard::config::PrincipalStoreMode;
use chrono::Local;

#[test]
fn test_create_principal_store() {
    let store = PrincipalStore::new(PrincipalStoreMode::InMemory);

    assert_eq!(store.get_total_secrets(), 0);
    assert_eq!(store.get_total_expirations(), 0);
}

#[test]
fn test_add_and_get_secret() {
    let mut store = PrincipalStore::new(PrincipalStoreMode::InMemory);

    let secret = SecretData {
        id: String::from("secret_id"),
        payload: b"payload".to_vec(),
        expiration_date: Local::now().timestamp() as u64 + 10000,
    };

    store.add_secret(secret.clone());

    let retrieved_secret = store.get_secret(&String::from("secret_id"));
    assert!(retrieved_secret.is_some());
    assert_eq!(retrieved_secret.unwrap().id, "secret_id");
}

#[test]
fn test_remove_secret() {
    let mut store = PrincipalStore::new(PrincipalStoreMode::InMemory);

    let secret = SecretData {
        id: String::from("secret_id"),
        payload: b"payload".to_vec(),
        expiration_date: Local::now().timestamp() as u64 + 10000,
    };

    store.add_secret(secret.clone());

    let removed = store.remove_secret(&String::from("secret_id"));
    assert!(removed);

    let retrieved_secret = store.get_secret(&String::from("secret_id"));
    assert!(retrieved_secret.is_none());
}

#[test]
fn test_clean_expired_secrets() {
    let mut store = PrincipalStore::new(PrincipalStoreMode::InMemory);

    let expired_secret = SecretData {
        id: String::from("expired_secret"),
        payload: b"expired_payload".to_vec(),
        expiration_date: Local::now().timestamp() as u64 - 1000,
    };

    let valid_secret = SecretData {
        id: String::from("valid_secret"),
        payload: b"valid_payload".to_vec(),
        expiration_date: Local::now().timestamp() as u64 + 10000,
    };

    store.add_secret(expired_secret.clone());
    store.add_secret(valid_secret.clone());

    store.clean_expired_secrets(10, 1);

    let expired_retrieved = store.get_secret(&String::from("expired_secret"));
    assert!(expired_retrieved.is_none());

    let valid_retrieved = store.get_secret(&String::from("valid_secret"));
    assert!(valid_retrieved.is_some());
}

#[test]
fn test_save_and_load_file_storage() {
    let test_file_path = "/tmp/test_principal_store.bin";

    let mut store = PrincipalStore::new(PrincipalStoreMode::FileStorage(String::from(test_file_path)));

    let secret = SecretData {
        id: String::from("file_secret"),
        payload: b"file_payload".to_vec(),
        expiration_date: Local::now().timestamp() as u64 + 10000,
    };

    store.add_secret(secret.clone());

    let new_store = PrincipalStore::new(PrincipalStoreMode::FileStorage(String::from(test_file_path)));

    let retrieved_secret = new_store.get_secret(&String::from("file_secret"));
    assert!(retrieved_secret.is_some());
    assert_eq!(retrieved_secret.unwrap().id, "file_secret");

    std::fs::remove_file(test_file_path).expect("Failed to delete test file");
}
