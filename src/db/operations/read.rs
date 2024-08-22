use crate::db::storage::principal_store::PrincipalStore;
use crate::db::models::secret_data::SecretData;

pub fn read_secret(store: &mut PrincipalStore, id: &str) -> Option<SecretData> {
    if let Some(secret_data) = store.get_secret(&id.to_string()).cloned() {
        if secret_data.is_expired() {
            store.remove_secret(&id.to_string());
            None
        } else {
            Some(secret_data)
        }
    } else {
        None
    }
}
