use std::{fs, path::Path};

pub fn init_repo() {
    let repo_path = ".chron";

    if Path::new(repo_path).exists() {
        println!("Repository already exists.");
        return;
    }

    fs::create_dir(repo_path).expect("Failed to create repo");
    fs::create_dir(".chron/objects").expect("Failed to create objects directory");
    fs::create_dir(".chron/commits").expect("Failed to create commits directory");

    fs::write(".chron/HEAD", "ref: main").expect("Failed to create HEAD");

    println!("Initialized empty Chronicon repository.");
}
