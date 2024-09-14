extern crate ephemeral_guard;

use ephemeral_guard::db::core::DatabaseCore;
use ephemeral_guard::config::PrincipalStoreMode;

#[test]
fn test_create_database_core() {
    let db_core = DatabaseCore::new(PrincipalStoreMode::InMemory);

    assert_eq!(db_core.get_total_expirations(), 0);
}

#[test]
fn test_generate_random_credential() {
    let credential = DatabaseCore::generate_random_credential();

    assert_eq!(credential.encryption_iv.len(), 16);
    assert_eq!(credential.encryption_key.len(), 16);
}

#[test]
fn test_create_secret() {
    let db_core = DatabaseCore::new(PrincipalStoreMode::InMemory);
    let credential = DatabaseCore::generate_random_credential();
    let expiration_seconds = 3600;

    let secret = db_core
        .create_secret("MySecretMessage", expiration_seconds, &credential)
        .expect("Failed to create secret");

    assert!(!secret.id.is_empty());
    assert_eq!(secret.payload.len(), 16);
}

#[test]
fn test_read_secret() {
    let db_core = DatabaseCore::new(PrincipalStoreMode::InMemory);
    let credential = DatabaseCore::generate_random_credential();
    let expiration_seconds = 3600;

    let secret = db_core
        .create_secret("MySecretMessage", expiration_seconds, &credential)
        .expect("Failed to create secret");

    let read_secret = db_core
        .read_secret(&secret.id)
        .expect("Failed to read secret");

    assert!(read_secret.is_some());
    assert_eq!(read_secret.unwrap().id, secret.id);
}

#[test]
fn test_read_expired_secret() {
    let db_core = DatabaseCore::new(PrincipalStoreMode::InMemory);
    let credential = DatabaseCore::generate_random_credential();
    let expiration_seconds = 1;

    let secret = db_core
        .create_secret("MySecretMessage", expiration_seconds, &credential)
        .expect("Failed to create secret");

    std::thread::sleep(std::time::Duration::from_secs(2));

    let read_secret = db_core
        .read_secret(&secret.id)
        .expect("Failed to read secret");

    assert!(read_secret.is_none());
}

#[test]
fn test_delete_secret() {
    let db_core = DatabaseCore::new(PrincipalStoreMode::InMemory);
    let credential = DatabaseCore::generate_random_credential();
    let expiration_seconds = 3600;

    let secret = db_core
        .create_secret("MySecretMessage", expiration_seconds, &credential)
        .expect("Failed to create secret");

    let deleted = db_core
        .delete_secret(&secret.id)
        .expect("Failed to delete secret");

    assert!(deleted);

    let read_secret = db_core
        .read_secret(&secret.id)
        .expect("Failed to read secret after deletion");

    assert!(read_secret.is_none());
}
