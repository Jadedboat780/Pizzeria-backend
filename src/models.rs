pub mod pizza;
pub mod user;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: i64,
}
