use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub published: bool,
    pub description: String,
    pub thumbnail: String,
    pub tags: Vec<String>,
    pub content: String,
}

impl Page {
    pub fn new(url: String, markdown_input: String, parser_options: Options) -> Self {
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
                tags.push(tag.as_string().unwrap().to_lowercase());
            }
        }

        let mut thumbnail: String = parsed_content.data.as_ref().unwrap()["thumbnail"]
            .as_string()
            .unwrap_or("".to_string());
        let thumbnail_url: Option<(&str, &str)> = url.rsplit_once("/");
        if !thumbnail.is_empty() && thumbnail_url.is_some() {
            thumbnail = format!("/{}/{}", thumbnail_url.unwrap().0, thumbnail);
        }

        Page {
            url: format!("/{}", url),
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
            description: parsed_content.data.as_ref().unwrap()["description"]
                .as_string()
                .unwrap_or("".to_string()),
            thumbnail,
            tags,
            content: html_output,
        }
    }
}
