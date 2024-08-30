use crate::log;
use crate::db::models::secret_data::SecretData;
use std::collections::{HashMap, VecDeque};
use chrono::Local;

#[derive(Debug)]
pub struct PrincipalStore {
    store: HashMap<String, SecretData>,
    expirations: VecDeque<(String, u64)>,
}

impl PrincipalStore {
    pub fn new() -> Self {
        PrincipalStore {
            store: HashMap::new(),
            expirations: VecDeque::new(),
        }
    }

    pub fn add_secret(&mut self, secret: SecretData) -> Option<SecretData> {
        let result = self.store.insert(secret.id.clone(), secret.clone());
        self.expirations.push_back((secret.id.clone(), secret.expiration_date));

        result
    }
    
    pub fn get_secret(&self, id: &String) -> Option<&SecretData> {
        self.store.get(id)
    }

    pub fn remove_secret(&mut self, id: &String) -> bool {
        log!("INFO", "Removing secret from store");
        self.store.remove(id).is_some()
    }

    pub fn get_total_expirations(&self) -> usize {
        self.expirations.len()
    }

    pub fn clean_expired_secrets(&mut self, batch_size: usize, current_batch: usize) {
        let total_expirations = self.expirations.len();
        if total_expirations == 0 {
            log!("INFO", "No expirations to clean.");
            return;
        }

        let batch_count = (total_expirations + batch_size - 1) / batch_size;
        if current_batch > batch_count {
            return;
        }

        let start_index = (current_batch - 1) * batch_size;
        let end_index = start_index + batch_size.min(total_expirations - start_index);

        let now = Local::now().timestamp().try_into().unwrap();
        let mut to_remove = Vec::new();

        for i in start_index..end_index {
            if let Some((_id, expiration)) = self.expirations.get(i) {
                if *expiration <= now {
                    to_remove.push(i);
                }
            }
        }

        for index in to_remove.iter().rev() {
            if let Some((id, _)) = self.expirations.remove(*index) {
                self.remove_secret(&id);
                log!("INFO", &format!("Secret with id {} has been removed due to expiration.", id));
            }
        }
    }
}