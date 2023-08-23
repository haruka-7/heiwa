use gray_matter::ParsedEntity;
use serde::Serialize;

#[derive(Serialize)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub author: Option<String>,
    pub date: Option<String>,
    pub published: Option<bool>,
    pub tags: Vec<String>,
}

impl Page {
    pub fn new(url: String, parsed_content: ParsedEntity) -> Self {
        Page {
            url,
            title: parsed_content.data.unwrap()["title"].as_string().unwrap(),
            author: Option::from("".to_string()),
            date: Option::from("".to_string()),
            published: Option::from(true),
            tags: Vec::new(),
        }
    }
}
