use crate::config::HASH_SEED;
use crate::crypto::{encryption, hash};

use super::credential::Credential;
use serde::{Serialize, Deserialize};

use std::fmt;
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretData {
    pub id: String,
    pub payload: Vec<u8>,
    pub expiration_date: u64,
}

impl SecretData{
    pub fn new(plaintext: &String, expiration_date: u64, credential: &Credential) -> Self {
        let key: [u8; 16] = credential.encryption_key.clone().try_into().expect("Invalid key length");
        let iv: [u8; 16] = credential.encryption_iv.clone().try_into().expect("Invalid IV length");
        
        let payload_encrypted = encryption::encrypt(&key, &iv, plaintext.as_bytes());
        
        let payload_encrypted_str = String::from_utf8_lossy(&payload_encrypted).to_string();
        let id = hash::generate_id(&payload_encrypted_str, *HASH_SEED);

        SecretData { id, payload: payload_encrypted, expiration_date }
    }

    pub fn decrypt(&self, credential: &Credential) -> String {
        let key: [u8; 16] = credential.encryption_key.clone().try_into().expect("Invalid key length");
        let iv: [u8; 16] = credential.encryption_iv.clone().try_into().expect("Invalid IV length");

        let decrypted_payload = encryption::decrypt(&key, &iv, self.payload.clone());
        let decrypted_payload_str = String::from_utf8_lossy(&decrypted_payload).to_string();

        decrypted_payload_str
    }

    pub fn is_expired(&self) -> bool {
        let now = Local::now();
        self.expiration_date < now.timestamp().try_into().unwrap()
    }
}

impl fmt::Display for SecretData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretData {{ id: {}, expiration_date: {} }}", self.id, self.expiration_date)
    }
}