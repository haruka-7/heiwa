use std::fs::{create_dir_all, File};
use std::io::Write;

use crate::configuration::Config;

pub fn init(project_name: String) {
    create_dir_all(format!("{}/pages", project_name)).unwrap();
    create_dir_all(format!("{}/themes", project_name)).unwrap();

    let mut config_file: File = File::create(format!("{}/config.toml", project_name)).unwrap();
    let config: Config = Config::default();
    let config_string: String = toml::to_string(&config).unwrap();
    config_file.write_all(config_string.as_bytes()).unwrap();

    let mut home_file: File = File::create(format!("{}/pages/home.md", project_name)).unwrap();
    let content = b"---\ntitle: Home\nauthor:\ndate:\npublished: false\ndescription: Personal blog\ntags:\n---\n# Welcome\nYou can edit this header by modifiyng the file `home.md`";
    home_file.write_all(content).unwrap();

    println!(
        "Project successfully created!\nNext steps : \n- download a theme in the \"themes\" directory\n- update the config.toml file\n- inside your project, launch the command : \"heiwa serve\"\nEnjoy!"
    );
}
