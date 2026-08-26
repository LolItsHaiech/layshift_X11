pub const DATA_DIR: &str = env!("DATA_DIR");

pub fn get_layout_dir() -> String {
    format!("{}/layouts/", DATA_DIR)
}

pub fn get_config_file() -> String {
    format!("{}/config.toml", DATA_DIR) // TODO: this address is only for testing, fix it before release
}
