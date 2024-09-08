// hash
pub const HASH_SEED: u32 = 1337;

// encryption
pub const ENCRYPTION_KEY: &[u8; 16] = b"This is the key!";
pub const ENCRYPTION_IV: &[u8; 16] = b"This is 16 bytes";

// server
pub enum DefaultTypeServer {
    TCP
}
pub const DEFAULT_SERVER: DefaultTypeServer = DefaultTypeServer::TCP;
pub const TCP_SERVER_PORT: &str = "1337";

// cleanup
pub const CLEANUP_INTERVAL_SECONDS: u64 = 20;