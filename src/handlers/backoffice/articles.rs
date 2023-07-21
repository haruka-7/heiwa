use crate::templates::DashboardTemplate;

// TODO protect these handlers

pub async fn list() -> DashboardTemplate {
    DashboardTemplate {
        name: "list".to_string(),
    }
}

pub async fn new() -> DashboardTemplate {
    DashboardTemplate {
        name: "new".to_string(),
    }
}

pub async fn new_action() -> DashboardTemplate {
    DashboardTemplate {
        name: "new".to_string(),
    }
}
