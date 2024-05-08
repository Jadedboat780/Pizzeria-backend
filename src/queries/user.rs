use sqlx::{postgres::PgQueryResult, query_file, query_file_as, PgPool};
use utils::encryption::hash_password;
use super::PgResult;
use crate::models::user::{User, CreateUser, CheckUserByEmail, CheckUserByUsername, PutUser, PatchUser};

pub async fn select_user_by_id(id: i32, pool: &PgPool) -> PgResult<Option<User>> {
    query_file_as!(
        User,
        "queries/user/select_user_by_id.sql",
       id
    )
        .fetch_optional(pool)
        .await
}

pub async fn select_user_by_email(user_data: &CheckUserByEmail, pool: &PgPool) -> PgResult<Option<User>> {
    query_file_as!(
        User,
        "queries/user/select_user_by_email.sql",
        user_data.email
    )
        .fetch_optional(pool)
        .await
}

pub async fn select_user_by_username(user_data: &CheckUserByUsername, pool: &PgPool) -> PgResult<Option<User>> {
    query_file_as!(
        User,
        "queries/user/select_user_by_username.sql",
        user_data.username
    )
        .fetch_optional(pool)
        .await
}

pub async fn insert_user(new_user: CreateUser, pool: &PgPool) -> PgResult<PgQueryResult> {
    let hash = hash_password(new_user.password.as_str()).await;
    query_file!(
        "queries/user/insert_user.sql",
        new_user.username,
        new_user.email,
        hash
    )
        .execute(pool)
        .await
}

pub async fn put_update_user(id: i32, update_data: PutUser, pool: &PgPool) -> PgResult<PgQueryResult> {
    query_file!(
        "queries/user/put_update_user.sql",
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

pub async fn patch_update_user(id: i32, update_data: PatchUser, pool: &PgPool) -> PgResult<PgQueryResult> {
    query_file!(
        "queries/user/patch_update_user.sql",
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

pub async fn delete_user(id: i32, pool: &PgPool) -> PgResult<PgQueryResult>  {
    query_file!(
        "queries/user/delete_user.sql",
        id
    )
        .execute(pool)
        .await
}