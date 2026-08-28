use serde::Deserialize;

pub const DATA_DIR: &str = env!("DATA_DIR");

#[derive(Debug, Deserialize)]
struct Config {
    source: String,
    target: String,
}

pub fn get_metadata_file() -> String {
    format!("{}/metadata.toml", get_layout_dir())
}

pub fn get_layout_dir() -> String {
    format!("{}/layouts/", DATA_DIR)
}
pub fn get_default_layouts() -> Result<(String, String), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(get_config_file())?;
    let config: Config = toml::from_str(&content)?;
    Ok((config.source, config.target))
}

pub fn get_config_file() -> String {
    format!("{}/config.toml", get_config_dir())
}

pub fn get_config_dir() -> String {
    let home = std::env::var("HOME").expect("HOME is not set!");
    format!("{}/.config/layshift", home)
}
