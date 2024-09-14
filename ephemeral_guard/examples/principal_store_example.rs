use ephemeral_guard::db::models::secret_data::SecretData;
use ephemeral_guard::db::models::credential::Credential;
use ephemeral_guard::db::storage::principal_store::PrincipalStore;
use ephemeral_guard::config::PrincipalStoreMode;
use chrono::Local;
use rand::Rng;

fn main() {
    // create storage
    // let mut storage = PrincipalStore::new();
    let mode = PrincipalStoreMode::InMemory;
    let mut storage = PrincipalStore::new(mode);
    println!("empty storage: {:?}", storage);
    
    // create secret data
    let plaintext_payload = "password: 84717__$ss1Az_2024".to_string();
    let current_time: u64 = Local::now().timestamp().try_into().unwrap();
    let expiration_date = current_time + 1000;
    
    let iv = (0..16).map(|_| rand::thread_rng().gen_range(0..255)).collect::<Vec<u8>>();
    let key = (0..16).map(|_| rand::thread_rng().gen_range(0..255)).collect::<Vec<u8>>();
    let credential = Credential::new(iv, key);

    let secret_data = SecretData::new(&plaintext_payload, expiration_date, &credential);
    let cloned_secret_data = secret_data.clone();
    
    // add secret to storage
    storage.add_secret(secret_data);
    println!("Almacenamiento: {:?}", storage);
    
    // get secret by id
    let get_secret_by_id = storage.get_secret(&cloned_secret_data.id);
    let secret_data_real = match get_secret_by_id {
        Some(secret) => {
            println!("Secreto encontrado: {:?}", secret);
            secret
        }
        None => {
            println!("Secreto no encontrado");
            return;
        }
    };
    
    // Read secret
    let secret_decrypted = secret_data_real.decrypt(&credential);
    println!("Secreto desencriptado: {}", secret_decrypted);

    // Remove secret
    storage.remove_secret(&cloned_secret_data.id);
    println!("Almacenamiento: {:?}", storage);
}
