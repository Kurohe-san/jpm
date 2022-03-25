use std::fs;
use serde::Deserialize;

const CONFIG_PATH: &str = "/etc/jpm/config.json";

#[derive(Deserialize)]
pub struct Config {
    pub elevated_privileges: String,
    pub package_db_path: String,
    pub mirrorlist: Vec<String>,
}

impl Config {

    // Load the config from the default location
    pub fn load_config() -> Config {
        let content = fs::read_to_string(CONFIG_PATH).expect(format!("Error loading file: {}", CONFIG_PATH).as_str());
        let ret: Config = serde_json::from_str(&content).unwrap();
        ret
    }

}