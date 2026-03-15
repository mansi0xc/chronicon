use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

pub fn create_commit(blob_hash: &str, message: &str) {
    let parent = read_head();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let commit_content = format!(
        "blob:{}\nparent:{}\nmessage:{}\ntimestamp:{}",
        blob_hash,
        parent.clone().unwrap_or("none".to_string()),
        message,
        timestamp,
    );

    let mut hasher = Sha256::new();
    hasher.update(commit_content.as_bytes());

    let commit_hash = format!("{:x}", hasher.finalize());
    let path = format!(".chron/commits/{}", commit_hash);

    fs::write(path, commit_content).expect("Failed to write commit");
    update_head(&commit_hash);

    println!("Created commit {}", commit_hash);
} 

fn read_head() -> Option<String>{
    let content = fs::read_to_string(".chron/HEAD").ok()?;
    if content.starts_with("ref:") {
        None
    } else {
        Some(content)
    }
}

fn update_head(hash: &str) {
    fs::write(".chron/HEAD", hash).expect("Failed to write to HEAD");
}
