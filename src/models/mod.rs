use serde::Deserialize;

pub mod pizza;
pub mod user;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: None,
            offset: 0,
        }
    }
}