use serde::{Serialize, Deserialize};
use config::{Config, File, FileFormat};
use once_cell::sync::Lazy;
use std::path::Path;


// hash
const DEFAULT_HASH_SEED: u32 = 1337;
pub static HASH_SEED: Lazy<u32> = Lazy::new(|| load_settings().hash_seed.unwrap_or(DEFAULT_HASH_SEED));

// server
pub enum DefaultTypeServer {
    TCP
}
pub const DEFAULT_SERVER: DefaultTypeServer = DefaultTypeServer::TCP;

const DEFAULT_TCP_SERVER_PORT: &str = "1337";
pub static TCP_SERVER_PORT: Lazy<String> = Lazy::new(|| load_settings().tcp_server_port.unwrap_or_else(|| DEFAULT_TCP_SERVER_PORT.to_string()));

// cleanup
const DEFAULT_CLEANUP_INTERVAL_SECONDS: u64 = 20;
pub static CLEANUP_INTERVAL_SECONDS: Lazy<u64> = Lazy::new(|| load_settings().cleanup_interval_seconds.unwrap_or(DEFAULT_CLEANUP_INTERVAL_SECONDS));

// Store
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PrincipalStoreMode {
    #[serde(skip)]
    InMemory,
    FileStorage(String),
}

// Log
const DEFAULT_LOG_ACTIVE: bool = false;
pub static LOG_ACTIVE: Lazy<bool> = Lazy::new(|| load_settings().log_active.unwrap_or(DEFAULT_LOG_ACTIVE));

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    hash_seed: Option<u32>,
    tcp_server_port: Option<String>,
    cleanup_interval_seconds: Option<u64>,
    log_active: Option<bool>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            hash_seed: Some(DEFAULT_HASH_SEED),
            tcp_server_port: Some(DEFAULT_TCP_SERVER_PORT.to_string()),
            cleanup_interval_seconds: Some(DEFAULT_CLEANUP_INTERVAL_SECONDS),
            log_active: Some(DEFAULT_LOG_ACTIVE),
        }
    }
}

fn load_settings() -> Settings {
    let mut settings = Config::default();

    if Path::new("config.toml").exists() {
        settings = Config::builder().add_source(File::new("config", FileFormat::Toml)).build().unwrap();
    }

    settings.try_deserialize::<Settings>().unwrap_or_default()
}
