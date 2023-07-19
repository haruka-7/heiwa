use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    pub name: String,
}

#[derive(Template)]
#[template(path = "pages/login.html")]
pub struct LoginTemplate {
    pub name: String,
}

#[derive(Template)]
#[template(path = "pages/register.html")]
pub struct RegisterTemplate {
    pub name: String,
}
