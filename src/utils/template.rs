use minify_html::{minify, Cfg};

pub fn minify_html(html_content: String) -> String {
    let code: &[u8] = html_content.as_bytes();
    let mut cfg = Cfg::new();
    cfg.do_not_minify_doctype = true;
    cfg.keep_closing_tags = true;
    cfg.keep_html_and_head_opening_tags = true;
    cfg.keep_spaces_between_attributes = true;
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;
    String::from_utf8(minify(code, &cfg)).unwrap()
}
