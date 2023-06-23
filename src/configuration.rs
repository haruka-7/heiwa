use serde::Deserialize;
use std::fs;
use toml::from_str;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub timeout: u64,
    pub log_level: String,
}

impl Config {
    pub fn new(config_toml_content: String) -> Self {
        from_str(&config_toml_content).unwrap_or(Self::default())
    }

    fn default() -> Self {
        Config {
            port: 3001,
            timeout: 5,
            log_level: "info".to_string(),
        }
    }
}

pub fn get_toml_content(toml_file_path: String) -> String {
    if fs::metadata(&toml_file_path).is_err() {
        tracing::error!("No TOML file found for path : {}", toml_file_path);
        "".to_string()
    } else {
        fs::read_to_string(toml_file_path).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn get_config_toml_content() -> String {
        "port = 8000
        timeout = 5
        log_level = \"debug\""
            .to_string()
    }

    #[test]
    fn test_config_constructor_new() {
        let config: Config = Config::new(get_config_toml_content());
        assert_eq!(config.port, 8000);
    }

    #[test]
    fn test_config_constructor_default() {
        let config: Config = Config::new("".to_string());
        assert_eq!(config.port, 3001);
    }
}
