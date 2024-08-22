use crate::db::models::secret_data::SecretData;
use crate::db::storage::principal_store::PrincipalStore;

pub fn create_secret(store: &mut PrincipalStore, plaintext: &str, expiration_date: u64) -> SecretData {
    let secret_data = SecretData::new(&plaintext.to_string(), expiration_date);

    store.add_secret(secret_data.clone());
    
    secret_data
}