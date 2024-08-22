use secret_db::config::{HASH_SEED};
use secret_db::crypto::{hash};

fn main() {
    // String to be hashed
    let text_to_hash = "son of a bitch".to_string();

    // Generate hash
    let hash = hash::generate_id(&text_to_hash, HASH_SEED);
    
    println!("Plain Text: {}", text_to_hash);
    println!("Hash: {}", hash);
}
