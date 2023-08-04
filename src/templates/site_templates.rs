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
#[template(path = "pages/error-page.html")]
pub struct ErrorPageTemplate {}
