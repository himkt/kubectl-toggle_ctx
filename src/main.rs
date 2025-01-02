use std::env;
use std::fs::{self, File, create_dir_all};
use std::path::PathBuf;

fn main() {
    let mut config_home_path = match env::var("XDG_CONFIG_HOME") {
        Ok(v) => PathBuf::from(v),
        Err(_) => {
            eprintln!("Environment variable XDG_CONFIG_HOME is not set.");
            return;
        }
    };

    config_home_path.push("kubectl-toggle-ctx");
    if !config_home_path.exists() {
        if let Err(e) = create_dir_all(&config_home_path) {
            eprintln!("Failed to create directory: {}", e);
            return;
        }
    }

    let config_path = config_home_path.join("hide");
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
