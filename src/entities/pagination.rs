use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: usize,
}
