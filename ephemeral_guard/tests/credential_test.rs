extern crate ephemeral_guard;

use ephemeral_guard::db::models::credential::Credential;

#[test]
fn test_credential_creation() {
    let iv = vec![1, 2, 3, 4];
    let key = vec![5, 6, 7, 8];
    let credential = Credential::new(iv.clone(), key.clone());

    assert_eq!(credential.encryption_iv, iv);
    assert_eq!(credential.encryption_key, key);
}

#[test]
fn test_get_credential_string() {
    let iv = vec![0x12, 0x34, 0x56, 0x78];
    let key = vec![0x9a, 0xbc, 0xde, 0xf0];
    let result = Credential::get_credential_string(&iv, &key);

    assert_eq!(result, "12345678_9abcdef0");
}

#[test]
fn test_new_from_string() {
    let credential_str = String::from("1234567890abcdef1234567890abcdef_abcdef1234567890abcdef1234567890");
    let credential = Credential::new_from_string(&credential_str).unwrap();

    assert_eq!(credential.encryption_iv, vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef]);
    assert_eq!(credential.encryption_key, vec![0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90]);
}

#[test]
fn test_new_from_string_invalid_format() {
    let invalid_str = String::from("part1_part2_part3_part4");
    let result = Credential::new_from_string(&invalid_str);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Credential format is incorrect: expected two parts separated by '_', got 4");
}

#[test]
fn test_new_from_string_invalid_length() {
    let invalid_str = String::from("1234_5678");
    let result = Credential::new_from_string(&invalid_str);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Each part of the credential must be at least 32 characters long. Got lengths 4 and 4");
}
