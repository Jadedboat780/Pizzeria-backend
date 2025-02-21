pub mod pizza;
pub mod user;

pub type PgResult<T> = Result<T, sqlx::Error>;
