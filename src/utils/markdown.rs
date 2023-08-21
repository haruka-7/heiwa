use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use pulldown_cmark::{html, Options, Parser};
use std::fs::read_to_string;

pub fn markdown_to_html(
    html_output: &mut String,
    file_path: String,
    parser_options: Options,
) -> ParsedEntity {
    tracing::debug!("Read fil with path {}", file_path);
    let markdown_input: String = read_to_string(format!("./pages/{}.md", file_path))
        .expect("Should have been able to read the file");
    let matter = Matter::<YAML>::new();
    let result = matter.parse(markdown_input.as_str());

    let parser: Parser = Parser::new_ext(result.content.as_str(), parser_options);
    html::push_html(html_output, parser);

    result
}
