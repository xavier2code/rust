use libs::serde::Deserialize;
use libs::toml;
use libs::walkdir::WalkDir;
use std::path::Path;

pub fn init() {
    match load_config("config.toml") {
        Ok(config) => match process_file(&config) {
            Ok(_) => (),
            Err(e) => eprintln!("Error processing file: {}", e),
        },
        Err(e) => eprintln!("Error loading config: {}", e),
    }
}

#[derive(Debug, Deserialize)]
struct Settings {
    directory: String,
    replacement: String,
    filter_suffix: Vec<String>,
}

fn load_config(file_path: &str) -> Result<Settings, Box<dyn std::error::Error>> {
    match std::fs::read_to_string(file_path) {
        Ok(contents) => match toml::from_str(&contents) {
            Ok(config) => Ok(config),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

fn process_file(settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(&settings.directory) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => return Err(Box::new(e)),
        };
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy();
            for suffix in &settings.filter_suffix {
                if file_name.ends_with(suffix) {
                    match replace_content(&entry.path(), &settings.replacement) {
                        Ok(_) => (),
                        Err(e) => return Err(Box::new(e)),
                    }
                }
            }
        }
    }
    Ok(())
}

fn replace_content(file_path: &Path, replacement: &str) -> Result<(), Box<dyn std::error::Error>> {
    match std::fs::read_to_string(file_path) {
        Ok(contents) => {
            let new_contents = contents.replace("<!--", replacement);
            match std::fs::write(file_path, new_contents) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}
