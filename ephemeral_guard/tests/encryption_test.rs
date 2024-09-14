extern crate ephemeral_guard;

use ephemeral_guard::crypto::encryption::{encrypt, decrypt};

#[test]
fn test_encrypt() {
    let key: [u8; 16] = [1; 16];
    let iv: [u8; 16] = [0; 16];
    let plain_text = b"Hello, World!";

    let encrypted = encrypt(&key, &iv, plain_text);

    assert_ne!(encrypted, plain_text);
}

#[test]
fn test_decrypt() {
    let key: [u8; 16] = [1; 16];
    let iv: [u8; 16] = [0; 16];
    let plain_text = b"Hello, World!";

    let encrypted = encrypt(&key, &iv, plain_text);
    let decrypted = decrypt(&key, &iv, encrypted);

    assert_eq!(decrypted, plain_text);
}

#[test]
fn test_encrypt_decrypt_round_trip() {
    let key: [u8; 16] = [1; 16];
    let iv: [u8; 16] = [0; 16];
    let plain_text = b"This is a secret message!";

    let encrypted = encrypt(&key, &iv, plain_text);
    let decrypted = decrypt(&key, &iv, encrypted);

    assert_eq!(decrypted, plain_text);
}
