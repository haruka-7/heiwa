use std::fs::{create_dir_all, File};
use std::io::Write;

use crate::configuration::Config;

pub fn init(project_name: String) {
    println!("project name is : {project_name}");

    create_dir_all(format!("{}/pages", project_name)).unwrap();
    create_dir_all(format!("{}/themes/shizen", project_name)).unwrap();
    create_dir_all(format!("{}/medias", project_name)).unwrap();

    let mut f = File::create(format!("{}/config.toml", project_name)).unwrap();
    let config: Config = Config::default();
    let string: String = toml::to_string(&config).unwrap();
    println!("{string}");
    f.write_all(toml::to_string(&config).unwrap().as_bytes())
        .unwrap();
}
