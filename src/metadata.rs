use std::collections::HashMap;

use serde::Deserialize;

use crate::config;

#[derive(Debug, Deserialize)]
pub struct Language {
    pub name: String,
    pub symbol: String,
    pub layouts: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    languages: HashMap<String, Language>,
}

pub fn get_languages_list() -> Result<Vec<Language>, Box<dyn std::error::Error>> {
    let metadata = get_metadata()?;
    Ok(metadata.languages.into_values().collect())
}

pub fn get_language_layouts(language: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let metadata = get_metadata()?;

    for lang in metadata.languages.into_values() {
        if lang.name.to_lowercase() == language || lang.symbol == language {
            return Ok(lang.layouts);
        }
    }

    Err("Selected language not found!".into())
}

fn get_metadata() -> Result<Metadata, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(config::get_metadata_file())?;
    let metadata: Metadata = toml::from_str(&content)?;
    Ok(metadata)
}
