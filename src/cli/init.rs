use std::fs::{create_dir_all, File};
use std::io::Write;

use crate::configuration::Config;

pub fn init(project_name: String) {
    create_dir_all(format!("{}/pages", project_name)).unwrap();
    create_dir_all(format!("{}/themes", project_name)).unwrap();
    create_dir_all(format!("{}/medias", project_name)).unwrap();

    let mut f = File::create(format!("{}/config.toml", project_name)).unwrap();
    let config: Config = Config::default();
    let config_string: String = toml::to_string(&config).unwrap();
    f.write_all(config_string.as_bytes()).unwrap();

    println!(
        "Project successfully created!\nDownload a theme in the \"themes\" directory, update the config.toml file, add some pages and run the builtin webserver with the command : \"heiwa serve\""
    );
}
