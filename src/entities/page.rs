use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};
use std::fs::read_to_string;
use serde::Serialize;

#[derive(Serialize)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub author: Option<String>,
    pub date: Option<String>,
    pub published: Option<bool>,
    pub tags: Vec<String>,
    pub content: String,
}

impl Page {
    pub fn new(url: String, file_path: String, parser_options: Options) -> Self {
        tracing::debug!("Read fil with path {}", file_path);
        
        let markdown_input: String = read_to_string(file_path.clone()).expect(format!("Should have been able to read the file : {}", file_path).as_str());
        let matter = Matter::<YAML>::new();
        let parsed_content = matter.parse(markdown_input.as_str());

        let mut html_output: String = String::new();
        let parser: Parser = Parser::new_ext(parsed_content.content.as_str(), parser_options);
        html::push_html(&mut html_output, parser);
        
        Page {
            url,
            title: parsed_content.data.unwrap()["title"].as_string().unwrap(),
            author: Option::from("".to_string()),
            date: Option::from("".to_string()),
            published: Option::from(true),
            tags: Vec::new(),
            content: html_output, 
        }
    }
}
