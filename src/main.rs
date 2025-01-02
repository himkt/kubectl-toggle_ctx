use std::env;
use std::fs::{self, File, create_dir_all};
use std::path::PathBuf;

fn main() {
    let mut home_path = PathBuf::from(env::var("HOME").unwrap());
    home_path.push(".config/kubectl-toggle_ctx");
    if !home_path.exists() {
        if let Err(e) = create_dir_all(&home_path) {
            eprintln!("Failed to create directory: {}", e);
            return;
        }
    }

    let config_path = home_path.join("toggle");
    if config_path.exists() {
        if let Err(e) = fs::remove_file(&config_path) {
            eprintln!("Failed to delete file: {}", e);
        }
    } else {
        if let Err(e) = File::create(&config_path) {
            eprintln!("Failed to create file: {}", e);
        }
    }
}
