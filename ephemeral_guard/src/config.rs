use serde::{Serialize, Deserialize};

// hash
pub const HASH_SEED: u32 = 1337;

// server
pub enum DefaultTypeServer {
    TCP
}
pub const DEFAULT_SERVER: DefaultTypeServer = DefaultTypeServer::TCP;
pub const TCP_SERVER_PORT: &str = "1337";

// cleanup
pub const CLEANUP_INTERVAL_SECONDS: u64 = 20;

// Store
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PrincipalStoreMode {
    #[serde(skip)]
    InMemory,
    FileStorage(String),
}