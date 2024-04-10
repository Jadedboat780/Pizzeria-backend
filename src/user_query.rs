use sqlx::{postgres::PgQueryResult, query_file, query_file_as, PgPool};
use utils::encryption::hash_password;
use crate::requests::user::{CreateUser, GetUserByEmail, GetUserByUsername, UpdateUser};

type PgResult = Result<PgQueryResult, sqlx::Error>;

pub async fn create_user(new_user: CreateUser, pool: &PgPool) -> PgResult {
    let hash = hash_password(new_user.password.as_str()).await;
    query_file!(
        "queries/user/create_user.sql",
        new_user.username,
        new_user.email,
        hash
    )
        .execute(pool)
        .await
}

pub async fn get_user_by_email(user_data: &GetUserByEmail, pool: &PgPool) -> Result<Option<GetUserByEmail>, sqlx::Error> {
     query_file_as!(
        GetUserByEmail,
        "queries/user/select_user_by_email.sql",
        user_data.email
    )
        .fetch_optional(pool)
        .await
}

pub async fn get_user_by_username(user_data: &GetUserByUsername, pool: &PgPool) -> Result<Option<GetUserByUsername>, sqlx::Error> {
    query_file_as!(
        GetUserByUsername,
        "queries/user/select_user_by_username.sql",
        user_data.username
    )
        .fetch_optional(pool)
        .await
}

pub async fn update_user(id: i32, update_data: UpdateUser, pool: &PgPool) -> PgResult {
    query_file!(
        "queries/user/update_user.sql",
        update_data.username,
        update_data.email,
        update_data.password,
        update_data.address,
        update_data.phone,
        update_data.avatar_url,
        id
    )
        .execute(pool)
        .await
}
