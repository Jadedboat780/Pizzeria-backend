pub mod pizza;
pub mod user;

pub(self) type PgResult<T> = Result<T, sqlx::Error>;
