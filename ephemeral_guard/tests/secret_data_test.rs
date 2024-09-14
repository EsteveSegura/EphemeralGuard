extern crate ephemeral_guard;

use ephemeral_guard::crypto::hash;
use ephemeral_guard::db::models::credential::Credential;
use ephemeral_guard::db::models::secret_data::SecretData;
use ephemeral_guard::config::HASH_SEED;
use chrono::Local;

#[test]
fn test_secret_data_creation() {
    // Datos de prueba
    let plaintext = String::from("Sensitive information");
    let expiration_date = Local::now().timestamp() as u64 + 10000; // Fecha de expiración en el futuro
    let credential = Credential {
        encryption_iv: vec![0; 16],
        encryption_key: vec![1; 16],
    };

    // Crear instancia de SecretData
    let secret_data = SecretData::new(&plaintext, expiration_date, &credential);

    // Verificar que el ID fue generado correctamente (hashear el texto encriptado)
    let encrypted_payload_str = String::from_utf8_lossy(&secret_data.payload).to_string();
    let expected_id = hash::generate_id(&encrypted_payload_str, HASH_SEED);
    assert_eq!(secret_data.id, expected_id);

    // Verificar que el payload fue encriptado
    assert_ne!(secret_data.payload, plaintext.as_bytes().to_vec());

    // Verificar que la fecha de expiración es la correcta
    assert_eq!(secret_data.expiration_date, expiration_date);
}
#[test]
fn test_secret_data_decryption() {
    // Datos de prueba
    let plaintext = String::from("Sensitive information");
    let expiration_date = Local::now().timestamp() as u64 + 10000;
    let credential = Credential {
        encryption_iv: vec![0; 16],
        encryption_key: vec![1; 16],
    };

    // Crear instancia de SecretData y luego desencriptar
    let secret_data = SecretData::new(&plaintext, expiration_date, &credential);
    let decrypted_text = secret_data.decrypt(&credential);

    // Verificar que el texto desencriptado es igual al texto plano original
    assert_eq!(decrypted_text, plaintext);
}

#[test]
fn test_secret_data_is_expired() {
    let plaintext = String::from("Sensitive information");

    // Simular una fecha de expiración pasada
    let expired_date = Local::now().timestamp() as u64 - 10000; // Fecha en el pasado
    let credential = Credential {
        encryption_iv: vec![0; 16],
        encryption_key: vec![1; 16],
    };

    let secret_data = SecretData::new(&plaintext, expired_date, &credential);

    // Verificar que el dato ha expirado
    assert!(secret_data.is_expired());

    // Simular una fecha de expiración en el futuro
    let future_date = Local::now().timestamp() as u64 + 10000; // Fecha en el futuro
    let secret_data_future = SecretData::new(&plaintext, future_date, &credential);

    // Verificar que el dato no ha expirado
    assert!(!secret_data_future.is_expired());
}

#[test]
fn test_secret_data_display() {
    let plaintext = String::from("Sensitive information");
    let expiration_date = Local::now().timestamp() as u64 + 10000;
    let credential = Credential {
        encryption_iv: vec![0; 16],
        encryption_key: vec![1; 16],
    };

    let secret_data = SecretData::new(&plaintext, expiration_date, &credential);
    let display_string = format!("{}", secret_data);

    // Verificar el formato de salida de Display
    assert!(display_string.contains(&secret_data.id));
    assert!(display_string.contains(&expiration_date.to_string()));
}
