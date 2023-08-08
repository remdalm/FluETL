use std::env;

pub fn get_env(name: &str) -> Result<String, String> {
    match env::var(name) {
        Ok(val) => Ok(val),
        Err(e) => Err(format!(
            "Error: {}\nPlease make sure the {} environment variable is set correctly.",
            e, name
        )),
    }
}

#[cfg(test)]
pub mod tests {
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    pub fn create_temp_env_file() -> NamedTempFile {
        NamedTempFile::new().expect("Failed to create temp .env file")
    }

    pub fn write_to_temp_env_file(temp_file: &NamedTempFile, content: &str) {
        let mut file = File::create(temp_file.path()).expect("Failed to write to temp .env file");
        writeln!(file, "{}", content).expect("Failed to write to temp .env file");
    }
}
