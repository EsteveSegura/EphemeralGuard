
use secret_db::db::models::secret_data::SecretData;
use secret_db::db::models::credential::Credential;
use chrono::Local;
use rand::Rng;

fn main() {
    let iv = (0..16).map(|_| rand::thread_rng().gen_range(0..255)).collect::<Vec<u8>>();
    let key = (0..16).map(|_| rand::thread_rng().gen_range(0..255)).collect::<Vec<u8>>();

    let credential = Credential::new(iv,key);
    println!("{:?}", credential);
    
    let plaintext_payload = "password: 84717__$ss1Az_2024".to_string();
    let current_time:u64 = Local::now().timestamp().try_into().unwrap();
    let expiration_date = current_time + 1000;

    let secret_data = SecretData::new(&plaintext_payload, expiration_date, &credential);

    println!("SecretData: {:?}", secret_data);

    println!("is_expired: {}", secret_data.is_expired());
    if secret_data.is_expired() {
        println!("Expired secret.");
    } else {
        println!("Valid secret --- Decrypted: {}", secret_data.decrypt(&credential));
    }
}
