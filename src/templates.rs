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
#[template(path = "pages/backoffice/dashboard.html")]
pub struct BackDashboardTemplate {
    pub name: String,
}

#[derive(Template)]
#[template(path = "pages/backoffice/articles-list.html")]
pub struct BackArticlesListTemplate {
    pub alert: String,
}

#[derive(Template)]
#[template(path = "pages/backoffice/articles-new.html")]
pub struct BackArticleNewTemplate {
    pub alert: String,
}

#[derive(Template)]
#[template(path = "pages/error-page.html")]
pub struct ErrorPageTemplate {}
