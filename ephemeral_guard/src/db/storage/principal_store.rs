use crate::log;
use crate::db::models::secret_data::SecretData;
use crate::config::PrincipalStoreMode;
use std::collections::{HashMap, VecDeque};
use chrono::Local;
use std::fs::File;
use std::io::{Write, Read, BufWriter, BufReader};
use std::path::Path;
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrincipalStore {
    store: HashMap<String, SecretData>,
    expirations: VecDeque<(String, u64)>,
    mode: PrincipalStoreMode,
}

impl PrincipalStore {
    pub fn new(mode: PrincipalStoreMode) -> Self {
        let mut store = PrincipalStore {
            store: HashMap::new(),
            expirations: VecDeque::new(),
            mode,
        };

        let mode_clone = store.mode.clone();

        if let PrincipalStoreMode::FileStorage(ref path) = mode_clone {
            store.load_from_file(path).expect("Failed to load data from file");
        }

        store
    }

    pub fn get_total_secrets(&self) -> usize {
        self.store.len()
    }

    pub fn add_secret(&mut self, secret: SecretData) -> Option<SecretData> {
        let result = self.store.insert(secret.id.clone(), secret.clone());
        self.expirations.push_back((secret.id.clone(), secret.expiration_date));

        if let PrincipalStoreMode::FileStorage(ref path) = self.mode {
            self.save_to_file(path).expect("Failed to save data to file");
        }

        log!("INFO", "Secret added to store");
        result
    }
    
    pub fn get_secret(&self, id: &String) -> Option<&SecretData> {
        self.store.get(id)
    }

    pub fn remove_secret(&mut self, id: &String) -> bool {
        let result = self.store.remove(id).is_some();
        log!("INFO", "Removing secret from store");
        
        if let PrincipalStoreMode::FileStorage(ref path) = self.mode {
            self.save_to_file(path).expect("Failed to save data to file");
        }
        
        result
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
        let mut writer = BufWriter::new(file);

        let serialized_data = serialize(&self.store).map_err(|e| format!("Failed to serialize data: {}", e))?;
        writer.write_all(&serialized_data).map_err(|e| format!("Failed to write to file: {}", e))?;
        writer.flush().map_err(|e| format!("Failed to flush writer: {}", e))?;

        Ok(())
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        if !Path::new(path).exists() {
            return Ok(());
        }

        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut reader = BufReader::new(file);

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).map_err(|e| format!("Failed to read from file: {}", e))?;

        self.store = deserialize(&buffer).map_err(|e| format!("Failed to deserialize data: {}", e))?;
        Ok(())
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