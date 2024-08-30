use std::sync::{Arc, RwLock};
use chrono::Local;
use rand::Rng;

use crate::db::storage::principal_store::PrincipalStore;
use crate::db::models::secret_data::SecretData;
use crate::log;

use crate::db::models::credential::Credential;

pub struct DatabaseCore {
    store: Arc<RwLock<PrincipalStore>>,
}

impl DatabaseCore {
    pub fn new() -> Self {
        DatabaseCore {
            store: Arc::new(RwLock::new(PrincipalStore::new())),
        }
    }

    pub fn generate_random_credential() -> Credential {
        let iv = (0..16).map(|_| rand::thread_rng().gen_range(0..255)).collect::<Vec<u8>>();
        let key = (0..16).map(|_| rand::thread_rng().gen_range(0..255)).collect::<Vec<u8>>();

        Credential::new(iv,key)
    }

    pub fn create_secret(&self, plaintext: &str, expiration_seconds: u64, credential: &Credential) -> Result<SecretData, String> {
        let now = Local::now();
        let current_timestamp:u64 = now.timestamp().try_into().map_err(|_| "Failed to convert timestamp")?;

        let expiration_date = current_timestamp + expiration_seconds;
        let secret = SecretData::new(&plaintext.to_string(), expiration_date, credential);
        
        let mut store = self.store.write().map_err(|_| "Failed to acquire write lock")?;
        store.add_secret(secret.clone());
        log!("INFO", &format!("CREATE secret with id: {}", secret.id));
        
        Ok(secret)
    }

    pub fn read_secret(&self, id: &str) -> Result<Option<SecretData>, String> {
        let mut store = self.store.write().map_err(|_| "Failed to acquire write lock")?;
        
        if let Some(secret) = store.get_secret(&id.to_string()).cloned() {
            if secret.is_expired() {
                log!("ERR", &format!("READ on secret: {}, but is expired", secret.id));
                store.remove_secret(&id.to_string());
                
                Ok(None)
            } else {
                log!("INFO", &format!("READ on secret: {}", secret.id));
                Ok(Some(secret))
            }
        } else {
            log!("ERR", &format!("READ on secret: {}, but not found", id));
            Ok(None)
        }
    }

    pub fn delete_secret(&self, id: &str) -> Result<bool, String> {
        let mut store = self.store.write().map_err(|_| "Failed to acquire write lock")?;
        let id_string = id.to_string();
        
        if store.get_secret(&id_string).is_some() {
            log!("INFO", &format!("DELETE on secret: {}", id));
            Ok(store.remove_secret(&id_string))
        } else {
            log!("ERR", &format!("DELETE on secret: {}, but not found", id));
            Ok(false)
        }
    }

    pub fn get_total_expirations(&self) -> usize {
        let store = self.store.read().unwrap();
        store.get_total_expirations()
    }

    pub fn clean_expired_secrets(&self, batch_size: usize, current_batch: usize) {
        let mut store = self.store.write().unwrap();
        store.clean_expired_secrets(batch_size, current_batch);
    }
}

impl Clone for DatabaseCore {
    fn clone(&self) -> Self {
        DatabaseCore {
            store: Arc::clone(&self.store),
        }
    }
}