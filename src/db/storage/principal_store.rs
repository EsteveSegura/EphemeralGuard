use crate::db::models::secret_data::SecretData;
use std::collections::HashMap;
use std::fmt;

pub struct PrincipalStore {
    store: HashMap<String, SecretData>,
}

impl PrincipalStore {
    pub fn new() -> Self {
        PrincipalStore {
            store: HashMap::new()
        }
    }

    pub fn add_secret(&mut self, secret: SecretData) -> Option<SecretData> {
        self.store.insert(secret.id.clone(), secret)
    }

    pub fn get_secret(&self, id: &String) -> Option<&SecretData> {
        self.store.get(id)
    }

    pub fn remove_secret(&mut self, id: &String) -> bool  {
        if self.store.get(id).is_some() {
            self.store.remove(id);
            return true;
        }

        false
    }
}

impl fmt::Debug for PrincipalStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrincipalStore")
            .field("store", &self.store)
            .finish()
    }
}