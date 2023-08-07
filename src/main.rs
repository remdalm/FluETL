use dotenvy;
use std::env;
use std::path::PathBuf;

mod domain;
mod infrastructure;

fn main() {
    // Check if a command-line argument is provided for the .env file path
    let args: Vec<String> = env::args().collect();
    let env_file_path = args
        .iter()
        .find(|&arg| arg.starts_with("--env-file="))
        .map(|arg| arg.split("=").nth(1).unwrap());

    // If --env argument is not provided, try to get .env file from the root of the crate
    if env_file_path.is_none() {
        let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let default_env_file = root_path.join(".env");

        if default_env_file.exists() {
            dotenvy::from_path(default_env_file).ok();
        } else {
            eprintln!(
                "Usage: {} --env-file=<env-file-path> or place .env file in the root of the crate",
                args[0]
            );
            std::process::exit(1);
        }
    } else {
        if let Err(e) = dotenvy::from_path(env_file_path.unwrap()) {
            eprintln!("Error loading .env file: {}", e);
            std::process::exit(1);
        }
    }
}
