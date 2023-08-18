use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub log_level: String,
    pub server_port: u16,
    pub server_timeout: u64,
    pub site: Site,
}

#[derive(Serialize, Deserialize)]
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
            server_port: 3000,
            server_timeout: 5,
            site: Site {
                title: "My Website".to_string(),
                comments: true,
            }
        }
    }
}
