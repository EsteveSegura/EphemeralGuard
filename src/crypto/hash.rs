use xxhash_rust::xxh32::xxh32;

pub fn generate_id(payload: &str, seed: u32) -> String {
    let hash = xxh32(payload.as_bytes(), seed);
    let hex_hash = format!("{:x}", hash);
    
    hex_hash
}