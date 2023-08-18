use std::fs::{create_dir_all, File};
use std::io::Write;

use crate::configuration::Config;

pub fn init(project_name: String) {
    create_dir_all(format!("{}/pages", project_name)).unwrap();
    create_dir_all(format!("{}/themes/shizen", project_name)).unwrap();
    create_dir_all(format!("{}/medias", project_name)).unwrap();

    let mut f = File::create(format!("{}/config.toml", project_name)).unwrap();
    let config: Config = Config::default();
    let config_string: String = toml::to_string(&config).unwrap();
    f.write_all(config_string.as_bytes()).unwrap();

    println!(
        "Project successfully created!\nAdd some pages to it and run the builtin server with :\n  cd {project_name}\n  heiwa serve"
    );
}
