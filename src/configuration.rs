use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub log_level: String,
    pub theme: String,
    pub site: Site,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    title: String,
    comments: bool,
}

impl Config {
    pub fn new(config_file_content: &str) -> Self {
        toml::from_str(config_file_content).unwrap()
    }

    pub fn default() -> Self {
        Config {
            log_level: "info".to_string(),
            theme: "shizen".to_string(),
            site: Site {
                title: "My Website".to_string(),
                comments: true,
            },
        }
    }
}
