use dotenvy;
use std::env;
use std::path::PathBuf;

mod domain;
mod infrastructure;

fn main() {
    let args: Vec<String> = env::args().collect();
    main_with_args(args, false).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}

fn main_with_args(args: Vec<String>, ignore_default_env_file: bool) -> Result<(), String> {
    let env_file_path = args
        .iter()
        .find(|&arg| arg.starts_with("--env-file="))
        .map(|arg| arg.split("=").nth(1).unwrap());

    // If --env argument is not provided, try to get .env file from the root of the crate
    if env_file_path.is_none() {
        let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let default_env_file = root_path.join(".env");

        if default_env_file.exists() && !ignore_default_env_file {
            dotenvy::from_path(default_env_file).map_err(|err| err.to_string())?;
        } else {
            return Err("Usage: program_name --env-file=<env-file-path> or place .env file in the root of the crate".to_string());
        }
    } else {
        let env_file_path = env_file_path.unwrap();
        dotenvy::from_path(env_file_path)
            .map_err(|err| format!("Error loading .env file: {}", err))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::infrastructure::environment::tests::{create_temp_env_file, write_to_temp_env_file};

    #[test]
    fn test_main_no_env_file() {
        // Arrange
        let args: Vec<String> = vec!["test_program".to_string()];

        // Act
        let result = super::main_with_args(args, true);

        // Assert
        assert!(
            result.is_err(),
            "Expected panic for missing --env-file argument"
        );
    }

    #[test]
    fn test_main_env_file_exists() {
        // Arrange
        let temp_file = create_temp_env_file();
        write_to_temp_env_file(&temp_file, "ENV_VAR_1=value1");
        let args: Vec<String> = vec![
            "test_program".to_string(),
            format!("--env-file={}", temp_file.path().display()),
        ];

        // Act
        let result = super::main_with_args(args, false);

        // Assert
        assert!(
            result.is_ok()
                && env::var("ENV_VAR_1").is_ok()
                && env::var("ENV_VAR_1").unwrap() == "value1",
            "Expected no panic when --env-file argument exists"
        );
    }
}
