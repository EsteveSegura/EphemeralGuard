
use secret_db::db::models::secret_data::{SecretData};
use secret_db::utils::time::current_timestamp;

fn main() {
    let plaintext_payload = "password: 84717__$ss1Az_2024".to_string();
    let expiration_date = current_timestamp() + 1000;

    let secret_data = SecretData::new(&plaintext_payload, expiration_date);

    println!("SecretData: {:?}", secret_data);

    println!("is_expired: {}", secret_data.is_expired());
    if secret_data.is_expired() {
        println!("Expired secret.");
    } else {
        println!("Valid secret --- Decrypted: {}", secret_data.decrypt());
    }
}
