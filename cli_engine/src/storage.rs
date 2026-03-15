use std::fs;

pub fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).expect("Failed to read path.")
}
