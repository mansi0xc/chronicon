use sha2::{Digest, Sha256};
use std::fs;

pub fn store_blob(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);

    let hash = format!("{:x}", hasher.finalize());
    let path = format!(".chron/objects/{}", hash);
    fs::write(path, data).expect("Failed to store object");

    hash
}
