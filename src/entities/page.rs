use gray_matter::ParsedEntity;

pub struct Page {
    pub title: String,
    pub author: Option<String>,
    pub date: Option<String>,
    pub published: Option<bool>,
    pub tags: Vec<String>
}

impl Page {
    pub fn new(parsed_content: ParsedEntity) -> Self {
        Page {
            title: parsed_content.data.unwrap()["title"].as_string().unwrap(),
            author: Option::from("".to_string()),
            date: Option::from("".to_string()),
            published: Option::from(true),
            tags: Vec::new(),
        }
    }
}
