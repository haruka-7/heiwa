use pulldown_cmark::{Options, Parser};
use std::fs::read_to_string;

pub fn markdown_parser(file_path: String, parser_options: Options) -> Parser<'static, 'static> {
    let markdown_input: String = read_to_string(format!("./pages/{}.md", file_path))
        .expect("Should have been able to read the file");
    Parser::new_ext(markdown_input.as_str(), parser_options)
}
