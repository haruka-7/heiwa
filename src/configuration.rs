use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub theme: String,
    pub comments: bool,
    pub articles_per_page: usize,
}

impl Config {
    pub fn new(config_file_content: &str) -> Self {
        toml::from_str(config_file_content).unwrap()
    }

    pub fn default() -> Self {
        Config {
            title: "Website title".to_string(),
            theme: "shizen".to_string(),
            comments: true,
            articles_per_page: 10,
        }
    }
}
