use tera::Context;
use crate::configuration::Config;

pub fn get_common_context(configuration: Config, title: Option<String>, description: Option<String>) -> Context {
    let mut context = Context::new();
    context.insert("meta_title", title.unwrap_or(configuration.site.title.clone()).as_str());
    context.insert("meta_description", description.unwrap_or(configuration.site.description).as_str());
    context.insert("site_title", configuration.site.title.as_str());
    context
}

pub fn minify_html(html_content: String) {}
