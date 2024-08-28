use libaes::Cipher;
use crate::log;

pub fn encrypt(key: &[u8; 16], iv: &[u8; 16], plain_text: &[u8]) -> Vec<u8> {    
    let cipher = Cipher::new_128(key);
    
    let encrypted = cipher.cbc_encrypt(iv, plain_text);
    log!("INFO", "Encryption successful");
    encrypted
}

pub fn decrypt(key: &[u8; 16], iv: &[u8; 16], cipher_text: Vec<u8>) -> Vec<u8> {
    let cipher = Cipher::new_128(key);
    
    let decrypted = cipher.cbc_decrypt(iv, &cipher_text);
    log!("INFO", "Decryption successful");
    decrypted
}