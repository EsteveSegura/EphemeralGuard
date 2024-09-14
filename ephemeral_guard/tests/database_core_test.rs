extern crate ephemeral_guard; // Nombre de tu crate

use ephemeral_guard::db::core::DatabaseCore;
use ephemeral_guard::config::PrincipalStoreMode;

#[test]
fn test_create_database_core() {
    // Crear DatabaseCore en modo memoria
    let db_core = DatabaseCore::new(PrincipalStoreMode::InMemory);

    // Verificar que el DatabaseCore se crea sin errores y que no tiene expiraciones
    assert_eq!(db_core.get_total_expirations(), 0);
}

#[test]
fn test_generate_random_credential() {
    // Generar una credencial aleatoria
    let credential = DatabaseCore::generate_random_credential();

    // Verificar que la longitud de IV y clave de cifrado es 16 bytes
    assert_eq!(credential.encryption_iv.len(), 16);
    assert_eq!(credential.encryption_key.len(), 16);
}

#[test]
fn test_create_secret() {
    let db_core = DatabaseCore::new(PrincipalStoreMode::InMemory);
    let credential = DatabaseCore::generate_random_credential();
    let expiration_seconds = 3600; // 1 hora

    // Crear un secreto
    let secret = db_core
        .create_secret("MySecretMessage", expiration_seconds, &credential)
        .expect("Failed to create secret");

    // Verificar que el secreto tiene el ID y la fecha de expiración correcta
    assert!(!secret.id.is_empty());
    assert_eq!(secret.payload.len(), 16); // Verificamos que el payload tiene longitud encriptada
}

#[test]
fn test_read_secret() {
    let db_core = DatabaseCore::new(PrincipalStoreMode::InMemory);
    let credential = DatabaseCore::generate_random_credential();
    let expiration_seconds = 3600;

    // Crear un secreto
    let secret = db_core
        .create_secret("MySecretMessage", expiration_seconds, &credential)
        .expect("Failed to create secret");

    // Leer el secreto desde el ID
    let read_secret = db_core
        .read_secret(&secret.id)
        .expect("Failed to read secret");

    // Verificar que el secreto se lee correctamente
    assert!(read_secret.is_some());
    assert_eq!(read_secret.unwrap().id, secret.id);
}

#[test]
fn test_read_expired_secret() {
    let db_core = DatabaseCore::new(PrincipalStoreMode::InMemory);
    let credential = DatabaseCore::generate_random_credential();
    let expiration_seconds = 1; // Expira en 1 segundo

    // Crear un secreto que expira pronto
    let secret = db_core
        .create_secret("MySecretMessage", expiration_seconds, &credential)
        .expect("Failed to create secret");

    // Esperar al menos 2 segundos para asegurarnos de que el secreto haya expirado
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Intentar leer el secreto expirado
    let read_secret = db_core
        .read_secret(&secret.id)
        .expect("Failed to read secret");

    // Verificar que el secreto ya ha expirado y ha sido eliminado
    assert!(read_secret.is_none());
}

#[test]
fn test_delete_secret() {
    let db_core = DatabaseCore::new(PrincipalStoreMode::InMemory);
    let credential = DatabaseCore::generate_random_credential();
    let expiration_seconds = 3600;

    // Crear un secreto
    let secret = db_core
        .create_secret("MySecretMessage", expiration_seconds, &credential)
        .expect("Failed to create secret");

    // Eliminar el secreto
    let deleted = db_core
        .delete_secret(&secret.id)
        .expect("Failed to delete secret");

    // Verificar que se eliminó correctamente
    assert!(deleted);

    // Verificar que el secreto ya no existe
    let read_secret = db_core
        .read_secret(&secret.id)
        .expect("Failed to read secret after deletion");

    assert!(read_secret.is_none());
}
