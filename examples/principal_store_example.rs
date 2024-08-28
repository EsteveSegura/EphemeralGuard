use secret_db::db::models::secret_data::SecretData;
use secret_db::db::models::credential::Credential;
use secret_db::db::storage::principal_store;
use chrono::Local;
use rand::Rng;

fn main() {
    // Create storage
    let mut storage = principal_store::PrincipalStore::new();
    println!("Empty storage: {:?}", storage);
    
    // Create secret
    let plaintext_payload = "password: 84717__$ss1Az_2024".to_string();
    let current_time:u64 = Local::now().timestamp().try_into().unwrap();
    let expiration_date = current_time + 1000;
    
    let iv = (0..16).map(|_| rand::thread_rng().gen_range(0..255)).collect::<Vec<u8>>();
    let key = (0..16).map(|_| rand::thread_rng().gen_range(0..255)).collect::<Vec<u8>>();
    let credential = Credential::new(iv,key);

    let secret_data = SecretData::new(&plaintext_payload, expiration_date, &credential);
    let cloned_secret_data = secret_data.clone();
    
    storage.add_secret(secret_data);
    println!("Storage: {:?}", storage);
    
    let get_secret_by_id = storage.get_secret(&cloned_secret_data.id);
    let secret_data_real = match get_secret_by_id {
        Some(secret) => {
            println!("Secret found: {:?}", secret);
            secret
        }
        None => {
            println!("Secret not found");
            return;
        }
    };
    println!("Secret found: {:?}", secret_data_real);
    
    let secret_decrypted = secret_data_real.decrypt(&credential);
    println!("Secret decrypted: {}", secret_decrypted);
    
    storage.remove_secret(&cloned_secret_data.id);
    println!("Storage: {:?}", storage);
    
}
