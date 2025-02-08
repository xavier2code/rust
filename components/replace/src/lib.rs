// This library provides functionality to batch process files with specified suffixes.
// It replaces a specified content in these files with a given replacement string.
// Usage:
// - Ensure the `config.toml` file is correctly configured with the `directory`, `replacement`, and `filter_suffix` fields.
// - Run the application to process the files according to the configuration.

mod content_replacer;

use serde::Deserialize;
use libs::toml;
use libs::walkdir::WalkDir;
use std::path::Path;

/// Initialize function, loads configuration and processes files.
/// If an error occurs during loading configuration or processing files, it prints the error message.
pub fn init() {
    if let Err(e) = load_config("config.toml").and_then(process_file) {
        eprintln!("Error: {}", e);
    }
}

#[derive(Debug, Deserialize)]
struct Settings {
    directory: String,
    replacement: String,
    filter_suffix: Vec<String>,
}

/// Loads configuration file from the specified path.
///
/// # Parameters
/// * `file_path` - Path to the configuration file.
///
/// # Returns
/// * `Settings` struct on success, error on failure.
fn load_config(file_path: &str) -> Result<Settings, Box<dyn std::error::Error + 'static>> {
    match std::fs::read_to_string(file_path) {
        Ok(contents) => match toml::from_str(&contents) {
            Ok(config) => Ok(config),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

/// Processes files in the specified directory according to the configuration.
///
/// # Parameters
/// * `settings` - Configuration containing directory path, replacement string, and file suffix filter.
///
/// # Returns
/// * `()` on success, error on failure.
fn process_file(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(&settings.directory)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy();
            if settings
                .filter_suffix
                .iter()
                .any(|suffix| file_name.ends_with(suffix))
            {
                replace_content(&entry.path(), &settings.replacement)?;
            }
        }
    }
    Ok(())
}

/// Replaces specified content in the file.
///
/// # Parameters
/// * `file_path` - Path to the file.
/// * `replacement` - Content to replace.
///
/// # Returns
/// * `()` on success, error on failure.
fn replace_content(file_path: &Path, replacement: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file_path)?;
    let new_contents = contents.replace("test", replacement);
    println!("Replaced content in file: {:?}", file_path);
    println!("New content: {}", new_contents);
    std::fs::write(file_path, new_contents)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use libs::tempfile::tempdir;
    use std::fs;

    /// Tests loading configuration file functionality.
    #[test]
    fn test_load_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        fs::write(
            &config_path,
            r#"
                directory = "test_dir"
                replacement = "test_replacement"
                filter_suffix = ["txt", "md"]
            "#,
        )
        .unwrap();

        let settings = load_config(config_path.to_str().unwrap()).unwrap();
        assert_eq!(settings.directory, "test_dir");
        assert_eq!(settings.replacement, "test_replacement");
        assert_eq!(settings.filter_suffix, vec!["txt", "md"]);
    }

    /// Tests processing files functionality.
    #[test]
    fn test_process_file() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        fs::write(
            &config_path,
            r#"
                directory = "test_dir"
                replacement = "test_replacement"
                filter_suffix = ["txt"]
            "#,
        )
        .unwrap();

        let test_dir = temp_dir.path().join("test_dir");
        fs::create_dir(&test_dir).unwrap();
        let test_file_path = test_dir.join("test.txt");
        fs::write(&test_file_path, "test_content").unwrap();

        let settings = load_config(config_path.to_str().unwrap()).unwrap();
        process_file(settings).unwrap();

        let new_content = fs::read_to_string(test_file_path).unwrap();
        assert_eq!(new_content, "test_content");
    }

    /// Tests replacing file content functionality.
    #[test]
    fn test_replace_content() {
        let temp_dir = tempdir().unwrap();
        let test_file_path = temp_dir.path().join("test.txt");
        fs::write(&test_file_path, "test_content").unwrap();

        replace_content(&test_file_path, "test_replacement").unwrap();

        let new_content = fs::read_to_string(test_file_path).unwrap();
        assert_eq!(new_content, "test_replacement_content");
    }
}
