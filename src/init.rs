use std::fs;
use std::io::Result;

pub fn init(project_name: String) {
    println!("project name is : {project_name}");
    create_config(&project_name).unwrap();
    create_dir(&project_name).unwrap();
}

fn create_config(project_name: &String) -> Result<()> {
    Ok(())
}

fn create_dir(project_name: &String) -> Result<()> {
    fs::create_dir_all(format!("{}/pages", project_name))?;
    fs::create_dir_all(format!("{}/themes/shizen", project_name))?;
    fs::create_dir_all(format!("{}/medias", project_name))?;
    Ok(())
}
