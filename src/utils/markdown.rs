use pulldown_cmark::{html, Options, Parser};
use std::fs::read_to_string;

pub fn markdown_to_html(html_output: &mut String, file_path: String, parser_options: Options) {
    let markdown_input: String = read_to_string(format!("./pages/{}.md", file_path))
        .expect("Should have been able to read the file");
    let parser: Parser = Parser::new_ext(markdown_input.as_str(), parser_options);
    html::push_html(html_output, parser);
}
