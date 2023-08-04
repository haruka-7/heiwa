use crate::templates::site_templates::HomeTemplate;

pub async fn show() -> HomeTemplate {
    HomeTemplate {
        name: "Minako".to_string(),
    }
}
