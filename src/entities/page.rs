use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};
use serde::Serialize;
use std::fs::read_to_string;

#[derive(Debug, Serialize)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub content: String,
}

impl Page {
    pub fn new(url: String, file_path: String, parser_options: Options) -> Self {
        tracing::debug!("Read fil with path {}", file_path);

        let markdown_input: String = read_to_string(file_path.clone())
            .unwrap_or_else(|_| panic!("Should have been able to read the file : {}", file_path));
        let matter = Matter::<YAML>::new();
        let parsed_content = matter.parse(markdown_input.as_str());

        let mut html_output: String = String::new();
        let parser: Parser = Parser::new_ext(parsed_content.content.as_str(), parser_options);
        html::push_html(&mut html_output, parser);

        let mut tags: Vec<String> = Vec::new();
        let parsed_tags = parsed_content.data.as_ref().unwrap()["tags"]
            .as_vec()
            .unwrap_or(vec![]);
        if !parsed_tags.is_empty() {
            for tag in parsed_tags {
                tags.push(tag.as_string().unwrap());
            }
        }

        Page {
            url,
            title: parsed_content.data.as_ref().unwrap()["title"]
                .as_string()
                .unwrap_or("".to_string()),
            author: parsed_content.data.as_ref().unwrap()["author"]
                .as_string()
                .unwrap_or("".to_string()),
            date: parsed_content.data.as_ref().unwrap()["date"]
                .as_string()
                .unwrap_or("".to_string()),
            published: parsed_content.data.as_ref().unwrap()["published"]
                .as_bool()
                .unwrap_or(true),
            tags,
            content: html_output,
        }
    }
}
