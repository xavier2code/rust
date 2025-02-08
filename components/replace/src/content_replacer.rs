// content_replacer
//
//
//
//

use libs::toml::from_str;
use serde::Deserialize;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
struct Settings {
    directory: String,
    replacement: String,
    filter_suffix: Vec<String>,
}

/// # load_config
/// - Loads configuration from a TOML file.
fn load_config(file_path: &str) -> Result<Settings, Box<dyn Error + 'static>> {
    match read_to_string(file_path) {
        Ok(contents) => match from_str(&contents) {
            Ok(config) => Ok(config),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_config() {
        let config = load_config("config.toml").unwrap();
        assert_eq!(config.directory, ".");
        assert_eq!(config.replacement, "");
        assert_eq!(config.filter_suffix, vec!["md".to_string(), "toml".to_string()]);
    }
}
