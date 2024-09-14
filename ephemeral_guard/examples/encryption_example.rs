use ephemeral_guard::crypto::encryption;

fn main() {
    pub const ENCRYPTION_KEY: &[u8; 16] = b"This is the key!";
    pub const ENCRYPTION_IV: &[u8; 16] = b"This is 16 bytes";

    let plaintext = "A plaintext".to_string();
    println!("String to encrypt (String): {}", plaintext);

    //encrypt
    let plaintext_bytes = plaintext.as_bytes();
    println!("String to encrypt (u8): {:?}", plaintext_bytes);

    let encrypted_text = encryption::encrypt(&ENCRYPTION_KEY, &ENCRYPTION_IV, plaintext_bytes);
    println!("encrypted_text value (vector<u8>): {:?}", encrypted_text);
    
    // decrypt
    let decrypted_text = encryption::decrypt(&ENCRYPTION_KEY, &ENCRYPTION_IV, encrypted_text);
    println!("decrypted_text value (vector<u8>): {:?}", decrypted_text);
    match String::from_utf8(decrypted_text) {
        Ok(string) => println!("decrypted_text value (String): {}", string),
        Err(e) => println!("Error ocurred when getting string: {}", e)
    }
}
