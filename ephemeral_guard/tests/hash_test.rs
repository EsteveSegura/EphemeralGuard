extern crate ephemeral_guard; // Nombre de tu crate

use ephemeral_guard::crypto::hash::generate_id;

#[test]
fn test_generate_id_same_payload_same_seed() {
    let payload = "Hello, World!";
    let seed = 12345;

    // Generar el hash dos veces para el mismo payload y seed
    let hash1 = generate_id(payload, seed);
    let hash2 = generate_id(payload, seed);

    // Verificar que los hashes sean iguales
    assert_eq!(hash1, hash2);
}

#[test]
fn test_generate_id_different_payloads() {
    let payload1 = "Hello, World!";
    let payload2 = "Goodbye, World!";
    let seed = 12345;

    // Generar hashes para diferentes payloads
    let hash1 = generate_id(payload1, seed);
    let hash2 = generate_id(payload2, seed);

    // Verificar que los hashes sean diferentes
    assert_ne!(hash1, hash2);
}

#[test]
fn test_generate_id_different_seeds() {
    let payload = "Hello, World!";
    let seed1 = 12345;
    let seed2 = 54321;

    // Generar hashes para el mismo payload pero con diferentes seeds
    let hash1 = generate_id(payload, seed1);
    let hash2 = generate_id(payload, seed2);

    // Verificar que los hashes sean diferentes
    assert_ne!(hash1, hash2);
}

#[test]
fn test_generate_id_format() {
    let payload = "Hello, World!";
    let seed = 12345;

    // Generar el hash
    let hash = generate_id(payload, seed);

    // Verificar que el hash esté en formato hexadecimal
    assert!(hash.chars().all(|c| c.is_digit(16)));
}
