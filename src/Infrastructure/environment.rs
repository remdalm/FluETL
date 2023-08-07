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
