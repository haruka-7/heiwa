use crate::templates::{LoginTemplate, RegisterTemplate};

pub async fn login() -> LoginTemplate {
    LoginTemplate {
        name: "login".to_string(),
    }
}

pub async fn register() -> RegisterTemplate {
    RegisterTemplate {
        name: "register".to_string(),
    }
}
