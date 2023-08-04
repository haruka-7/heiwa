use crate::entities::articles_entity::Article;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    pub name: String,
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
#[template(path = "pages/dashboard/dashboard.html")]
pub struct BackDashboardTemplate {
    pub name: String,
}

#[derive(Template)]
#[template(path = "pages/dashboard/articles-list.html")]
pub struct BackArticlesListTemplate {
    pub alert: String,
    pub articles: Vec<Article>,
}

#[derive(Template)]
#[template(path = "pages/dashboard/articles-new.html")]
pub struct BackArticleNewTemplate {
    pub alert: String,
    pub author_id: i32,
}

#[derive(Template)]
#[template(path = "pages/error-page.html")]
pub struct ErrorPageTemplate {}
