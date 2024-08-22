use crate::config::{HASH_SEED, ENCRYPTION_KEY, ENCRYPTION_IV};
use crate::utils::time::current_timestamp;
use crate::crypto::{encryption, hash};
use std::fmt;

#[derive(Debug, Clone)]
pub struct SecretData {
    pub id: String,
    pub payload: Vec<u8>,
    pub expiration_date: u64,
}

impl SecretData{
    pub fn new(plaintext: &String, expiration_date: u64) -> Self {
        let payload_encrypted = encryption::encrypt(&ENCRYPTION_KEY, &ENCRYPTION_IV, plaintext.as_bytes());
        
        let payload_encrypted_str = String::from_utf8_lossy(&payload_encrypted).to_string();
        let id = hash::generate_id(&payload_encrypted_str, HASH_SEED);

        SecretData { id, payload: payload_encrypted, expiration_date }
    }

    pub fn decrypt(&self) -> String {
        let decrypted_payload = encryption::decrypt(&ENCRYPTION_KEY, &ENCRYPTION_IV, self.payload.clone());
        let decrypted_payload_str = String::from_utf8_lossy(&decrypted_payload).to_string();

        decrypted_payload_str
    }

    pub fn is_expired(&self) -> bool {
        self.expiration_date < current_timestamp()
    }
}

impl fmt::Display for SecretData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretData {{ id: {}, expiration_date: {} }}", self.id, self.expiration_date)
    }
}