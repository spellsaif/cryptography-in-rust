use sha2::{Sha256, Digest};


pub fn sha256_hash(input: &str) -> String {

    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    format!("{:x}", result)
}