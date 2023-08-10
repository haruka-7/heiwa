use askama::Template;
use crate::templates::site_templates::ArticleData;

#[derive(Template)]
#[template(path = "pages/dashboard/articles-list.html")]
pub struct DashboardArticlesListTemplate {
    pub alert: String,
    pub articles: Vec<ArticleData>,
}

#[derive(Template)]
#[template(path = "pages/dashboard/articles-edit.html")]
pub struct DashboardArticleNewTemplate {
    pub action: String,
    pub alert: String,
    pub author_id: i32,
    pub title: String,
    pub content: String,
    pub permalink: String,
    pub meta_description: String,
}

#[derive(Template)]
#[template(path = "pages/dashboard/profile.html")]
pub struct DashboardProfileTemplate {
    pub alert: String,
    pub author_id: i32,
    pub display_name: String,
    pub biography: String,
}
