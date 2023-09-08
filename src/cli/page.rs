use std::fs::File;
use std::io::Write;

pub fn page(name: String) {
    println!("Creating page {}...", name);
    let mut file: File = File::create(format!("{}.md", name)).unwrap();
    let content = b"---\ntitle: Title\nauthor:\ndate:\npublished:\nthumbnail:\ndescription:\ntags:\n---\n# Title\nPage content";
    file.write_all(content).unwrap();
}
