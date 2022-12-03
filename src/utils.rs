use std::env;
use std::fs;

pub fn read_file_from_current_folder(file: &str) -> String {
    let file_path = env::current_dir().unwrap().join(file);
    return fs::read_to_string(file_path).unwrap();
}