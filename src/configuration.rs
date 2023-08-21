use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub comments: bool,
    pub theme: String,
}

impl Config {
    pub fn new(config_file_content: &str) -> Self {
        toml::from_str(config_file_content).unwrap()
    }

    pub fn default() -> Self {
        Config {
            title: "Website title".to_string(),
            comments: true,
            theme: "shizen".to_string(),
        }
    }
}
