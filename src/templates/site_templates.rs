use crate::entities::articles_entity::Article;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    pub title: String,
    pub alert: String,
    pub articles: Vec<ArticleData>,
}

#[derive(Template)]
#[template(path = "pages/login.html")]
pub struct LoginTemplate {
    pub alert: String,
}

#[derive(Template)]
#[template(path = "pages/register.html")]
pub struct RegisterTemplate {
    pub alert: String,
}

#[derive(Template)]
#[template(path = "pages/error-page.html")]
pub struct ErrorPageTemplate {}

pub struct ArticleData {
    pub title: String,
    pub content: String,
    pub meta_description: String,
    pub author: String,
    pub permalink: String,
}

impl ArticleData {
    pub fn new(article: Article, author_name: String) -> Self {
        Self {
            title: article.title,
            content: article.content.unwrap_or("".to_string()),
            meta_description: article.meta_description.unwrap_or("".to_string()),
            author: author_name,
            permalink: article.permalink,
        }
    }
}
