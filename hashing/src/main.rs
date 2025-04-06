use sha2::{Sha256, Digest};


fn sha256_hash(input: &str) -> String {

    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    format!("{:x}", result)
}


fn main() {
    println!("Input: 'hello world'");
    let hash = sha256_hash("hello world");
    println!("SHA256 Hash: {}", hash);
}
